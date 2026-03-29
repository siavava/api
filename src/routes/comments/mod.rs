//! # Comments Route
//!
//! WebSocket endpoint for real-time comment operations.
//!
//! Exposes `GET /comments/` which upgrades to a WebSocket
//! connection. Each text frame is parsed as a [`CommentRequest`]
//! and dispatched to the appropriate handler; a
//! [`CommentResponse`] is sent back as JSON.
//!
//! Clients are also notified of changes made by any client.
//! Mutation events (create, edit, like, delete) on the client's
//! active route — triggered by *any* connected client (including
//! self) — are forwarded automatically via the broadcast channel.
//!
//! **Note:** The standalone `/comments/` endpoint does not support
//! setting an active route. Use the unified `/api/connect/`
//! endpoint with the `watch` scope for broadcast filtering.

pub mod handlers;

use crate::{AppState, models::comments::CommentEvent, protocol::socket};
use handlers::socket::handle_message;

use actix_web::{
  Error as ActixError, HttpRequest, HttpResponse, get,
  web::{Data, scope},
};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use tokio::sync::broadcast;
use tracing::info;

/// Registers the `/comments/` WebSocket endpoint.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(scope("/comments").service(comments_ws));
}

/// `GET /comments/` — WebSocket endpoint for real-time comment
/// operations.
///
/// # Behavior
///
/// 1. Upgrades the HTTP connection to a WebSocket.
/// 2. Each incoming text frame is parsed as a [`CommentRequest`]
///    (JSON with an `"action"` tag).
/// 3. The request is dispatched to the matching controller
///    function.
/// 4. For `List` requests, the response is sent directly back to
///    the client.
/// 5. For mutation requests (create, edit, like, delete), the
///    response is broadcast to all clients on the unified
///    `/api/connect/` endpoint whose active route matches the
///    affected path. Errors are sent directly to the requesting
///    client only.
///
/// Also handles `Ping`/`Pong` for keep-alive and logs client
/// disconnects.
///
/// # Example Request Frames
///
/// **Create a comment:**
/// ```json
/// {
///   "action": "create",
///   "comment": {
///     "text": "Hello!",
///     "markup": "<p>Hello!</p>",
///     "author": "Alice",
///     "path": "/blog/post-1"
///   }
/// }
/// ```
///
/// **Create a reply:**
/// ```json
/// {
///   "action": "create",
///   "comment": {
///     "text": "Reply!",
///     "markup": "<p>Reply!</p>",
///     "author": "Bob",
///     "path": "/blog/post-1"
///   },
///   "reply_to": "665a1b2c3d4e5f6a7b8c9d0e"
/// }
/// ```
///
/// **Edit a comment:**
/// ```json
/// {
///   "action": "edit",
///   "id": "665a1b2c3d4e5f6a7b8c9d0e",
///   "text": "Updated text"
/// }
/// ```
///
/// **Like a comment:**
/// ```json
/// {
///   "action": "like",
///   "id": "665a1b2c3d4e5f6a7b8c9d0e"
/// }
/// ```
///
/// **Delete a comment:**
/// ```json
/// {
///   "action": "delete",
///   "id": "665a1b2c3d4e5f6a7b8c9d0e"
/// }
/// ```
///
/// **List comments for a page:**
/// ```json
/// {
///   "action": "list",
///   "path": "/blog/post-1"
/// }
/// ```
///
/// # Example Response Frames
///
/// **Created:**
/// ```json
/// {
///   "type": "created",
///   "comment": {
///     "id": "665a...",
///     "text": "Hello!",
///     "markup": "<p>Hello!</p>",
///     "author": "Alice",
///     "path": "/blog/post-1",
///     "created_time": "2025-06-01T12:00:00Z",
///     "likes": 0,
///     "replies": []
///   }
/// }
/// ```
///
/// **List:**
/// ```json
/// {
///   "type": "list",
///   "comments": [
///     {
///       "id": "665a...",
///       "text": "Hello!",
///       "replies": [
///         {
///           "id": "665b...",
///           "replies": []
///         }
///       ]
///     }
///   ]
/// }
/// ```
///
/// **Error:**
/// ```json
/// {
///   "type": "error",
///   "message": "invalid message: ..."
/// }
/// ```
#[get("/")]
async fn comments_ws(
  req: HttpRequest,
  stream: actix_web::web::Payload,
  app_state: Data<AppState>,
) -> Result<HttpResponse, ActixError> {
  let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;
  let db_client = app_state.db_client.clone();
  let broadcast_tx = app_state.comment_events.clone();
  let broadcast_rx = broadcast_tx.subscribe();

  actix_web::rt::spawn(ws_event_loop(
    session,
    msg_stream,
    db_client,
    broadcast_tx,
    broadcast_rx,
  ));

  Ok(response)
}

/// Main event loop for a single WebSocket client.
///
/// Multiplexes incoming client messages with broadcast events
/// from the shared channel. Runs until the client disconnects or
/// a send error occurs.
async fn ws_event_loop(
  mut session: Session,
  mut msg_stream: actix_ws::MessageStream,
  db_client: mongodb::Client,
  broadcast_tx: broadcast::Sender<CommentEvent>,
  mut _broadcast_rx: broadcast::Receiver<CommentEvent>,
) {
  loop {
    tokio::select! {
      ws_msg = msg_stream.next() => {
        let Some(Ok(msg)) = ws_msg else { break };
        if !handle_ws_frame(
          msg, &db_client, &mut session,
          &broadcast_tx,
        ).await {
          break;
        }
      }
    }
  }
}

/// Processes a single incoming WebSocket frame.
///
/// Returns `false` if the connection should be closed.
async fn handle_ws_frame(
  msg: Message,
  db_client: &mongodb::Client,
  session: &mut Session,
  broadcast_tx: &broadcast::Sender<CommentEvent>,
) -> bool {
  match msg {
    Message::Text(text) => {
      let (response, event_path) = handle_message(db_client, &text).await;

      if let Some(path) = event_path {
        let _ = broadcast_tx.send(CommentEvent { path, response });
      } else {
        return socket::send_json(session, &response).await;
      }
      true
    }
    Message::Ping(bytes) => session.pong(&bytes).await.is_ok(),
    Message::Close(_) => {
      info!("ws client disconnected");
      false
    }
    _ => true,
  }
}
