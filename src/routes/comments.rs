use crate::{
  AppState,
  controllers::comments,
  models::comments::{WsRequest, WsResponse},
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
) -> WsResponse {
  let request: WsRequest = match serde_json::from_str(text) {
    Ok(req) => req,
    Err(e) => {
      return WsResponse::Error {
        message: format!("invalid message: {e}"),
      };
    }
  };

  match request {
    WsRequest::Create { comment, reply_to } => {
      let parent_oid = match reply_to {
        Some(ref id_str) => match ObjectId::parse_str(id_str) {
          Ok(oid) => Some(oid),
          Err(e) => {
            return WsResponse::Error {
              message: format!("invalid reply_to id: {e}"),
            };
          }
        },
        None => None,
      };
      match comments::create_comment(db_client, comment, parent_oid.as_ref()).await {
        Ok(created) => WsResponse::Created { comment: created },
        Err(e) => WsResponse::Error {
          message: format!("failed to create comment: {e}"),
        },
      }
    }

    WsRequest::Edit { id, edit } => {
      let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(e) => {
          return WsResponse::Error {
            message: format!("invalid id: {e}"),
          };
        }
      };
      match comments::edit_comment(db_client, &oid, edit).await {
        Ok(Some(updated)) => WsResponse::Updated { comment: updated },
        Ok(None) => WsResponse::Error {
          message: "comment not found".into(),
        },
        Err(e) => WsResponse::Error {
          message: format!("failed to edit comment: {e}"),
        },
      }
    }

    WsRequest::Delete { id } => {
      let oid = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(e) => {
          return WsResponse::Error {
            message: format!("invalid id: {e}"),
          };
        }
      };
      match comments::delete_comment(db_client, &oid).await {
        Ok(true) => WsResponse::Deleted { id },
        Ok(false) => WsResponse::Error {
          message: "comment not found".into(),
        },
        Err(e) => WsResponse::Error {
          message: format!("failed to delete comment: {e}"),
        },
      }
    }

    WsRequest::List { path } => match comments::list_comments(db_client, &path).await {
      Ok(list) => WsResponse::List { comments: list },
      Err(e) => WsResponse::Error {
        message: format!("failed to list comments: {e}"),
      },
    },
  }
}
