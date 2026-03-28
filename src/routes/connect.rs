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
  controllers::{opengraph, views},
  models::{
    comments::{CommentEvent, CommentResponse},
    connect::{ClientChannels, ConnectRequest, ConnectResponse, OpenGraphResponse},
    health::HealthDiagnostics,
    playback::PlaybackResponse,
    views::ViewsResponse,
  },
  protocol::socket,
  routes::{
    comments::handlers::socket as comment_handlers,
    playback::handlers::socket as playback_handlers, views::handlers::socket as view_handlers,
  },
};

use actix_web::{Error as ActixError, HttpRequest, HttpResponse, get, web::Data};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use std::sync::{
  Arc,
  atomic::{AtomicUsize, Ordering},
};
use tokio::sync::broadcast;
use tracing::info;

/// Registers the `/connect/` WebSocket endpoint.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(connect_ws);
}

/// `GET /api/connect/` — unified WebSocket endpoint.
#[get("/connect/")]
async fn connect_ws(
  req: HttpRequest,
  stream: actix_web::web::Payload,
  app_state: Data<AppState>,
) -> Result<HttpResponse, ActixError> {
  let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;
  let channels = ClientChannels::from_app_state(&app_state);
  let active_clients = app_state.active_clients.clone();

  // Increment active client count and notify all clients.
  let count = active_clients.fetch_add(1, Ordering::Relaxed) + 1;
  let _ = channels.senders.active_count.send(count);

  actix_web::rt::spawn(ws_event_loop(
    session,
    msg_stream,
    app_state.into_inner().as_ref().clone(),
    channels,
    active_clients,
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
  app_state: AppState,
  channels: ClientChannels,
  active_clients: Arc<AtomicUsize>,
) {
  let ClientChannels {
    senders,
    mut receivers,
  } = channels;

  let mut active_path: Option<String> = None;

  // Channel for receiving results from spawned background tasks
  // (e.g. OpenGraph fetches) without blocking the event loop.
  let (deferred_tx, mut deferred_rx) = tokio::sync::mpsc::channel::<ConnectResponse>(4);

  loop {
    tokio::select! {
      // Deliver results from background tasks.
      Some(response) = deferred_rx.recv() => {
        if !socket::send_json(&mut session, &response).await {
          break;
        }
      }

      ws_msg = msg_stream.next() => {
        let Some(Ok(msg)) = ws_msg else { break };
        let prev_path = active_path.clone();
        if !handle_ws_frame(
          msg, &app_state, &mut session,
          &senders.comments, &mut active_path, &deferred_tx,
        ).await {
          break;
        }

        if active_path != prev_path {
          views::track_page_view(&app_state.db_client, &senders, active_path.as_deref()).await;
        }
      }

      event = receivers.comments.recv() => {
        let Ok(event) = event else { continue };
        if active_path.as_deref() != Some(&event.path) {
          continue;
        }
        let response = ConnectResponse::Comments(event.response);
        if !socket::send_json(&mut session, &response).await {
          break;
        }
      }

      event = receivers.views.recv() => {
        let Ok(event) = event else { continue };
        // ? ALWAYS NOTIFY OF VIEWS UPDATES
        // ? TO CLIENTS LOOKING AT ARCHIVE
        let path = active_path.as_deref();
        let is_archive = path == Some("<b>:/archive");
        let is_match = path == Some(event.views.route.as_str());
        if !is_archive && !is_match {
          continue;
        }
        let response = ConnectResponse::Views(ViewsResponse::Update {
          views: event.views,
        });
        if !socket::send_json(&mut session, &response).await {
          break;
        }
      }

      Ok(count) = receivers.active_count.recv() => {
        let response = ConnectResponse::Views(ViewsResponse::ActiveCount { count });
        if !socket::send_json(&mut session, &response).await {
          break;
        }
      }
    }
  }

  // Client disconnected — decrement count and notify.
  let count = active_clients.fetch_sub(1, Ordering::Relaxed) - 1;
  let _ = senders.active_count.send(count);
}

/// Processes a single incoming WebSocket frame.
///
/// Returns `false` if the connection should be closed.
async fn handle_ws_frame(
  msg: Message,
  app_state: &AppState,
  session: &mut Session,
  comment_tx: &broadcast::Sender<CommentEvent>,
  active_path: &mut Option<String>,
  deferred_tx: &tokio::sync::mpsc::Sender<ConnectResponse>,
) -> bool {
  let db_client = &app_state.db_client;
  match msg {
    Message::Text(text) => {
      let request = match ConnectRequest::parse(&text) {
        Ok(req) => req,
        Err(e) => {
          let response = ConnectResponse::Comments(CommentResponse::Error { message: e });
          return socket::send_json(session, &response).await;
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
            return socket::send_json(session, &wrapped).await;
          }
          true
        }
        ConnectRequest::Views(views_req) => {
          view_handlers::handle_ws_request(db_client, session, views_req).await
        }
        ConnectRequest::Health(options) => {
          let diagnostics = HealthDiagnostics::collect(app_state, &options).await;
          let response = ConnectResponse::Health(diagnostics);
          socket::send_json(session, &response).await
        }
        ConnectRequest::Playback(req) => {
          let spotify = app_state.spotify.clone();
          let tx = deferred_tx.clone();
          actix_web::rt::spawn(async move {
            let response = match spotify {
              Some(ref client) => playback_handlers::handle_request(client, req).await,
              None => PlaybackResponse::Error {
                message: "spotify not configured".into(),
              },
            };
            let _ = tx.send(ConnectResponse::Playback(response)).await;
          });
          true
        }
        ConnectRequest::OpenGraph(req) => {
          // Spawn the fetch so it doesn't block the event loop.
          // The result is sent back via deferred_tx and delivered in the
          // select! loop's deferred_rx branch.
          let tx = deferred_tx.clone();
          actix_web::rt::spawn(async move {
            let og_response = match opengraph::fetch_opengraph(&req.url).await {
              Ok(data) => OpenGraphResponse::Data(data),
              Err(e) => OpenGraphResponse::Error { message: e },
            };
            let _ = tx.send(ConnectResponse::OpenGraph(og_response)).await;
          });
          true
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
