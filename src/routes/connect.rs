//! # Connect Route
//!
//! Unified WebSocket endpoint at `/api/connect/` that multiplexes
//! both comment operations and view-count watch subscriptions over
//! a single connection.
//!
//! The client's active path (set by a comment `List` request) is
//! used for both comment event filtering and view-count updates.

use crate::{
  AppState,
  controllers::views::{self, ViewsIncrement},
  models::comments::{CommentEvent, CommentResponse},
  models::connect::{ConnectRequest, ConnectResponse},
  models::views::{ViewEvent, ViewsRequest, ViewsResponse},
  routes::comments::handlers as comment_handlers,
};

use actix_web::{Error as ActixError, HttpRequest, HttpResponse, get, web::Data};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use tokio::sync::broadcast;
use tracing::{error, info};

/// Registers the `/connect/` WebSocket endpoint.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(connect_ws);
}

/// Serializes a [`ConnectResponse`] and sends it over the WebSocket.
///
/// Returns `false` if the send failed (connection should be closed).
async fn send_response(session: &mut Session, response: &ConnectResponse) -> bool {
  match serde_json::to_string(response) {
    Ok(json) => {
      if let Err(e) = session.text(json).await {
        error!("failed to send ws message: {e}");
        return false;
      }
      true
    }
    Err(e) => {
      error!("failed to serialize response: {e}");
      true
    }
  }
}

/// `GET /api/connect/` — unified WebSocket endpoint.
#[get("/connect/")]
async fn connect_ws(
  req: HttpRequest,
  stream: actix_web::web::Payload,
  app_state: Data<AppState>,
) -> Result<HttpResponse, ActixError> {
  let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;
  let db_client = app_state.db_client.clone();
  let comment_tx = app_state.comment_events.clone();
  let comment_rx = comment_tx.subscribe();
  let view_rx = app_state.view_events.subscribe();

  let view_tx = app_state.view_events.clone();
  let active_clients = app_state.active_clients.clone();
  let active_count_tx = app_state.active_count_events.clone();
  let active_count_rx = active_count_tx.subscribe();

  // Increment active client count and notify all clients.
  let count = active_clients.fetch_add(1, Ordering::Relaxed) + 1;
  let _ = active_count_tx.send(count);

  actix_web::rt::spawn(ws_event_loop(
    session, msg_stream, db_client,
    comment_tx, comment_rx,
    view_tx, view_rx,
    active_clients, active_count_tx, active_count_rx,
  ));

  Ok(response)
}

/// Main event loop for a single WebSocket client.
///
/// Multiplexes incoming client messages with comment and view-count
/// broadcast events. The client's `active_path` (set by a comment
/// `List` request) controls which events are forwarded.
async fn ws_event_loop(
  mut session: Session,
  mut msg_stream: actix_ws::MessageStream,
  db_client: mongodb::Client,
  comment_tx: broadcast::Sender<CommentEvent>,
  mut comment_rx: broadcast::Receiver<CommentEvent>,
  view_tx: broadcast::Sender<ViewEvent>,
  mut view_rx: broadcast::Receiver<ViewEvent>,
  active_clients: Arc<AtomicUsize>,
  active_count_tx: broadcast::Sender<usize>,
  mut active_count_rx: broadcast::Receiver<usize>,
) {
  let mut active_path: Option<String> = None;

  loop {
    tokio::select! {
      ws_msg = msg_stream.next() => {
        let Some(Ok(msg)) = ws_msg else { break };
        let prev_path = active_path.clone();
        if !handle_ws_frame(
          msg, &db_client, &mut session,
          &comment_tx, &mut active_path,
        ).await {
          break;
        }

        // When the active path changes, increment the view count
        // and broadcast the update to all clients via the WS channel.
        if active_path != prev_path
          && let Some(ref path) = active_path
          && let Ok(updated) = views::get_views(&db_client, path, ViewsIncrement::INCREMENT).await
        {
          let _ = view_tx.send(ViewEvent { views: updated });
        }
      }

      event = comment_rx.recv() => {
        let Ok(event) = event else { continue };
        if active_path.as_deref() != Some(&event.path) {
          continue;
        }
        let response = ConnectResponse::Comments(event.response);
        if !send_response(&mut session, &response).await {
          break;
        }
      }

      event = view_rx.recv() => {
        let Ok(event) = event else { continue };
        if active_path.as_deref() != Some(event.views.route.as_str()) {
          continue;
        }
        let response = ConnectResponse::Views(ViewsResponse::Update {
          views: event.views,
        });
        if !send_response(&mut session, &response).await {
          break;
        }
      }

      Ok(count) = active_count_rx.recv() => {
        let response = ConnectResponse::Views(ViewsResponse::ActiveCount { count });
        if !send_response(&mut session, &response).await {
          break;
        }
      }
    }
  }

  // Client disconnected — decrement count and notify.
  let count = active_clients.fetch_sub(1, Ordering::Relaxed) - 1;
  let _ = active_count_tx.send(count);
}

/// Processes a single incoming WebSocket frame.
///
/// Returns `false` if the connection should be closed.
async fn handle_ws_frame(
  msg: Message,
  db_client: &mongodb::Client,
  session: &mut Session,
  comment_tx: &broadcast::Sender<CommentEvent>,
  active_path: &mut Option<String>,
) -> bool {
  match msg {
    Message::Text(text) => {
      let request = match ConnectRequest::parse(&text) {
        Ok(req) => req,
        Err(e) => {
          let response = ConnectResponse::Comments(CommentResponse::Error { message: e });
          return send_response(session, &response).await;
        }
      };

      match request {
        ConnectRequest::Comments(comment_req) => {
          let (response, event_path) =
            comment_handlers::handle_request(db_client, *comment_req, active_path).await;

          if let Some(path) = event_path {
            let _ = comment_tx.send(CommentEvent { path, response });
          } else {
            let wrapped = ConnectResponse::Comments(response);
            return send_response(session, &wrapped).await;
          }
          true
        }
        ConnectRequest::Views(views_req) => {
          handle_views_request(db_client, session, views_req).await
        }
      }
    }
    Message::Ping(bytes) => session.pong(&bytes).await.is_ok(),
    Message::Close(_) => {
      info!("ws client disconnected");
      false
    }
    _ => true,
  }
}

/// Handles a views-scoped request.
async fn handle_views_request(
  db_client: &mongodb::Client,
  session: &mut Session,
  request: ViewsRequest,
) -> bool {
  let response = match request {
    ViewsRequest::List => {
      let all = views::get_all_views(db_client).await.unwrap_or_default();
      ConnectResponse::Views(ViewsResponse::List { views: all })
    }
  };
  send_response(session, &response).await
}
