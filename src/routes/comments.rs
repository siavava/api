//! # Comments Route
//!
//! WebSocket endpoint for real-time comment operations.
//!
//! Exposes `GET /comments/` which upgrades to a WebSocket connection.
//! Each text frame is parsed as a [`CommentRequest`] and dispatched to the
//! appropriate controller function; a [`CommentResponse`] is sent back as JSON.

use crate::{
  AppState,
  controllers::comments,
  models::comments::{CommentRequest, CommentResponse},
};

use actix_web::{
  Error as ActixError, HttpRequest, HttpResponse, get,
  web::{Data, scope},
};
use actix_ws::Message;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use tracing::{error, info};

/// Registers the `/comments/` WebSocket endpoint.
///
/// # Arguments
///
/// * `cfg` — The Actix-Web service config to register routes on.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(scope("/comments").service(comments_ws));
}

/// Parses a hex string as an `ObjectId`.
///
/// # Arguments
///
/// * `id` — A hex-encoded ObjectId string (24 characters).
///
/// # Returns
///
/// * `Ok(ObjectId)` on success.
/// * `Err(CommentResponse::Error { .. })` with a descriptive message on
///   failure.
fn parse_oid(id: &str) -> Result<ObjectId, String> {
  ObjectId::parse_str(id).map_err(|e| format!("invalid id: {e}"))
}

/// `GET /comments/` — WebSocket endpoint for real-time comment operations.
///
/// # Behavior
///
/// 1. Upgrades the HTTP connection to a WebSocket.
/// 2. Each incoming text frame is parsed as a [`CommentRequest`]
///    (JSON with an `"action"` tag).
/// 3. The request is dispatched to the matching controller function.
/// 4. A [`CommentResponse`] is serialized as JSON and sent back.
///
/// Also handles `Ping`/`Pong` for keep-alive and logs client disconnects.
///
/// # Example Request Frames
///
/// **Create a comment:**
/// ```json
/// { "action": "create", "comment": { "text": "Hello!", "markup": "<p>Hello!</p>", "author": "Alice", "path": "/blog/post-1" } }
/// ```
///
/// **Create a reply:**
/// ```json
/// { "action": "create", "comment": { "text": "Reply!", "markup": "<p>Reply!</p>", "author": "Bob", "path": "/blog/post-1" }, "reply_to": "665a1b2c3d4e5f6a7b8c9d0e" }
/// ```
///
/// **Edit a comment:**
/// ```json
/// { "action": "edit", "id": "665a1b2c3d4e5f6a7b8c9d0e", "text": "Updated text" }
/// ```
///
/// **Like a comment:**
/// ```json
/// { "action": "like", "id": "665a1b2c3d4e5f6a7b8c9d0e" }
/// ```
///
/// **Delete a comment:**
/// ```json
/// { "action": "delete", "id": "665a1b2c3d4e5f6a7b8c9d0e" }
/// ```
///
/// **List comments for a page:**
/// ```json
/// { "action": "list", "path": "/blog/post-1" }
/// ```
///
/// # Example Response Frames
///
/// **Created:**
/// ```json
/// { "type": "created", "comment": { "id": "665a...", "text": "Hello!", "markup": "<p>Hello!</p>", "author": "Alice", "path": "/blog/post-1", "created_time": "2025-06-01T12:00:00Z", "likes": 0, "replies": [] } }
/// ```
///
/// **List:**
/// ```json
/// { "type": "list", "comments": [ { "id": "665a...", "text": "Hello!", ... , "replies": [ { "id": "665b...", ... , "replies": [] } ] } ] }
/// ```
///
/// **Error:**
/// ```json
/// { "type": "error", "message": "invalid message: ..." }
/// ```
#[get("/")]
async fn comments_ws(
  req: HttpRequest,
  stream: actix_web::web::Payload,
  app_state: Data<AppState>,
) -> Result<HttpResponse, ActixError> {
  let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;
  let db_client = app_state.db_client.clone();

  actix_web::rt::spawn(async move {
    while let Some(Ok(msg)) = msg_stream.next().await {
      match msg {
        Message::Text(text) => {
          let response = handle_message(&db_client, &text).await;
          match serde_json::to_string(&response) {
            Ok(json) => {
              if let Err(e) = session.text(json).await {
                error!("failed to send ws message: {e}");
                break;
              }
            }
            Err(e) => {
              error!("failed to serialize response: {e}");
            }
          }
        }
        Message::Ping(bytes) => {
          if session.pong(&bytes).await.is_err() {
            break;
          }
        }
        Message::Close(_) => {
          info!("ws client disconnected");
          break;
        }
        _ => {}
      }
    }
  });

  Ok(response)
}

/// Parses a raw WebSocket text frame and dispatches it to the matching
/// controller.
///
/// # Arguments
///
/// * `db_client` — The MongoDB client.
/// * `text` — The raw JSON string from the client.
///
/// # Returns
///
/// A [`CommentResponse`] — always succeeds at the Rust level; errors are
/// represented as [`CommentResponse::Error`].
async fn handle_message(db_client: &mongodb::Client, text: &str) -> CommentResponse {
  let request: CommentRequest = match serde_json::from_str(text) {
    Ok(req) => req,
    Err(e) => {
      return CommentResponse::Error {
        message: format!("invalid message: {e}"),
      };
    }
  };

  match request {
    CommentRequest::Create { comment, reply_to } => {
      let parent_oid = match reply_to {
        Some(ref id_str) => match parse_oid(id_str) {
          Ok(oid) => Some(oid),
          Err(e) => return CommentResponse::Error { message: e },
        },
        None => None,
      };
      match comments::create_comment(db_client, comment, parent_oid.as_ref()).await {
        Ok(created) => CommentResponse::Created { comment: created },
        Err(e) => CommentResponse::Error {
          message: format!("failed to create comment: {e}"),
        },
      }
    }

    CommentRequest::Edit { id, edit } => {
      let oid = match parse_oid(&id) {
        Ok(oid) => oid,
        Err(e) => return CommentResponse::Error { message: e },
      };
      match comments::edit_comment(db_client, &oid, edit).await {
        Ok(Some(updated)) => CommentResponse::Updated { comment: updated },
        Ok(None) => CommentResponse::Error {
          message: "comment not found".into(),
        },
        Err(e) => CommentResponse::Error {
          message: format!("failed to edit comment: {e}"),
        },
      }
    }

    CommentRequest::Like { id } => {
      let oid = match parse_oid(&id) {
        Ok(oid) => oid,
        Err(e) => return CommentResponse::Error { message: e },
      };
      match comments::like_comment(db_client, &oid).await {
        Ok(Some(liked)) => CommentResponse::Liked { comment: liked },
        Ok(None) => CommentResponse::Error {
          message: "comment not found".into(),
        },
        Err(e) => CommentResponse::Error {
          message: format!("failed to like comment: {e}"),
        },
      }
    }

    CommentRequest::Delete { id } => {
      let oid = match parse_oid(&id) {
        Ok(oid) => oid,
        Err(e) => return CommentResponse::Error { message: e },
      };
      match comments::delete_comment(db_client, &oid).await {
        Ok(deleted_count) if deleted_count > 0 => CommentResponse::Deleted { id, deleted_count },
        Ok(_) => CommentResponse::Error {
          message: "comment not found".into(),
        },
        Err(e) => CommentResponse::Error {
          message: format!("failed to delete comment: {e}"),
        },
      }
    }

    CommentRequest::List { path } => match comments::list_comments(db_client, &path).await {
      Ok(list) => CommentResponse::List { comments: list },
      Err(e) => CommentResponse::Error {
        message: format!("failed to list comments: {e}"),
      },
    },
  }
}
