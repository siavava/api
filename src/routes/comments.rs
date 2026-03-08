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

pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(scope("/comments").service(comments_ws));
}

/// Parse a hex string as an ObjectId, returning a CommentResponse::Error on failure.
fn parse_oid(id: &str) -> Result<ObjectId, CommentResponse> {
  ObjectId::parse_str(id).map_err(|e| CommentResponse::Error {
    message: format!("invalid id: {e}"),
  })
}

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

async fn handle_message(
  db_client: &mongodb::Client,
  text: &str,
) -> CommentResponse {
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
          Err(e) => return e,
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
        Err(e) => return e,
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
        Err(e) => return e,
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
        Err(e) => return e,
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
