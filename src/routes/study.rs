//! # Study Routes
//!
//! REST authentication (`/study/auth/*`) and an authenticated real-time
//! WebSocket (`/study/connect/`) for notes, annotations, and reading
//! progress. Entirely separate from the blog's `/connect/` endpoint — the
//! blog continues to work with no auth dependency.
//!
//! ## Endpoints
//!
//! | Method | Path                  | Description                          |
//! |--------|-----------------------|--------------------------------------|
//! | POST   | `/study/auth/signup/` | Register; returns `{ token, user }`. |
//! | POST   | `/study/auth/login/`  | Log in; returns `{ token, user }`.   |
//! | GET    | `/study/connect/`     | Authenticated WebSocket (`?token=`). |

use crate::{
  AppState,
  controllers::study,
  models::study::{
    AuthResponse, LoginRequest, PublicUser, SignupRequest, StudyEvent,
    StudyRequest, StudyResponse, StudySectionEvent,
  },
  protocol::socket,
};

use actix_web::{
  Error as ActixError, HttpRequest, HttpResponse, get, post,
  web::{Data, Json, Query, scope},
};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;
use tracing::info;

/// Registers the `/study` scope.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(
    scope("/study")
      .service(signup)
      .service(login)
      .service(connect_ws),
  );
}

// --------------------------------------------------------------------------- //
// REST auth
// --------------------------------------------------------------------------- //

/// `POST /study/auth/signup/`
#[post("/auth/signup/")]
async fn signup(
  app_state: Data<AppState>,
  body: Json<SignupRequest>,
) -> Result<HttpResponse, ActixError> {
  let SignupRequest { username, email, password } = body.into_inner();
  match study::create_user(&app_state.db_client, &username, &email, &password)
    .await
  {
    Ok(user) => match study::make_token(&app_state.jwt_secret, &user) {
      Ok(token) => Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: PublicUser::from(&user),
      })),
      Err(e) => Ok(HttpResponse::InternalServerError().json(err(&e))),
    },
    Err(e) => Ok(HttpResponse::BadRequest().json(err(&e))),
  }
}

/// `POST /study/auth/login/`
#[post("/auth/login/")]
async fn login(
  app_state: Data<AppState>,
  body: Json<LoginRequest>,
) -> Result<HttpResponse, ActixError> {
  let LoginRequest { identifier, password } = body.into_inner();
  let user = match study::find_user(&app_state.db_client, &identifier).await {
    Ok(Some(u)) => u,
    Ok(None) => {
      return Ok(HttpResponse::Unauthorized().json(err("invalid credentials")));
    }
    Err(e) => return Ok(HttpResponse::InternalServerError().json(err(&e))),
  };
  if !study::verify_password(&password, &user.password_hash) {
    return Ok(HttpResponse::Unauthorized().json(err("invalid credentials")));
  }
  match study::make_token(&app_state.jwt_secret, &user) {
    Ok(token) => Ok(HttpResponse::Ok().json(AuthResponse {
      token,
      user: PublicUser::from(&user),
    })),
    Err(e) => Ok(HttpResponse::InternalServerError().json(err(&e))),
  }
}

fn err(message: &str) -> serde_json::Value {
  serde_json::json!({ "message": message })
}

// --------------------------------------------------------------------------- //
// WebSocket
// --------------------------------------------------------------------------- //

#[derive(Debug, Deserialize)]
struct ConnectQuery {
  token: Option<String>,
}

/// `GET /study/connect/[?token=JWT]` — study WebSocket.
///
/// The token is **optional**: an anonymous connection may read public content
/// and subscribe to a section's live stream, but cannot mutate. A valid token
/// upgrades the same connection to an authed session (`user_id`/`username`
/// recorded server-side). Empty strings mark an anonymous session.
#[get("/connect/")]
async fn connect_ws(
  req: HttpRequest,
  stream: actix_web::web::Payload,
  query: Query<ConnectQuery>,
  app_state: Data<AppState>,
) -> Result<HttpResponse, ActixError> {
  let token = query.token.clone().unwrap_or_default();
  let (user_id, username) =
    match study::verify_token(&app_state.jwt_secret, &token) {
      Ok(c) => (c.sub, c.username),
      Err(_) => (String::new(), String::new()),
    };

  let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;
  actix_web::rt::spawn(study_event_loop(
    session,
    msg_stream,
    app_state.into_inner().as_ref().clone(),
    user_id,
    username,
  ));
  Ok(response)
}

/// Per-client event loop. Multiplexes incoming requests with two broadcast
/// streams: the per-user channel (this user's own sessions stay in sync) and
/// the section channel (public mutations reach everyone — authed or anonymous —
/// viewing the section this connection last subscribed to). `user_id` is empty
/// for anonymous sessions.
async fn study_event_loop(
  mut session: Session,
  mut msg_stream: actix_ws::MessageStream,
  app_state: AppState,
  user_id: String,
  username: String,
) {
  let mut events = app_state.study_events.subscribe();
  let mut section_events = app_state.study_section_events.subscribe();
  let mut current_section: Option<String> = None;

  loop {
    tokio::select! {
      ws_msg = msg_stream.next() => {
        let Some(Ok(msg)) = ws_msg else { break };
        match msg {
          Message::Text(text) => {
            if !handle_text(
              &app_state, &user_id, &username, &text,
              &mut session, &mut current_section,
            ).await {
              break;
            }
          }
          Message::Ping(bytes) => {
            if session.pong(&bytes).await.is_err() { break; }
          }
          Message::Close(_) => {
            info!("study ws client disconnected");
            break;
          }
          _ => {}
        }
      }

      event = events.recv() => {
        match event {
          Ok(StudyEvent { user_id: owner, response }) => {
            if owner != user_id { continue; }
            if !socket::send_json(&mut session, &response).await { break; }
          }
          Err(RecvError::Lagged(_)) => continue,
          Err(RecvError::Closed) => break,
        }
      }

      sect = section_events.recv() => {
        match sect {
          Ok(StudySectionEvent { section_path, response }) => {
            if current_section.as_deref() != Some(section_path.as_str()) {
              continue;
            }
            if !socket::send_json(&mut session, &response).await { break; }
          }
          Err(RecvError::Lagged(_)) => continue,
          Err(RecvError::Closed) => break,
        }
      }
    }
  }
}

/// Broadcasts a mutation to all of this user's live sessions (including the
/// requester — the select! loop delivers it back via the `events` branch).
fn broadcast(app_state: &AppState, user_id: &str, response: StudyResponse) {
  let _ = app_state.study_events.send(StudyEvent {
    user_id: user_id.to_string(),
    response,
  });
}

/// Broadcasts a public-content mutation to every session viewing `section_path`.
fn broadcast_section(
  app_state: &AppState,
  section_path: &str,
  response: StudyResponse,
) {
  let _ = app_state.study_section_events.send(StudySectionEvent {
    section_path: section_path.to_string(),
    response,
  });
}

/// Handles one text frame. Returns `false` if the connection should close.
/// `user_id`/`username` are empty for anonymous sessions; mutations are
/// rejected in that case.
async fn handle_text(
  app_state: &AppState,
  user_id: &str,
  username: &str,
  text: &str,
  session: &mut Session,
  current_section: &mut Option<String>,
) -> bool {
  let db = &app_state.db_client;
  let request = match StudyRequest::parse(text) {
    Ok(r) => r,
    Err(message) => {
      return socket::send_json(session, &StudyResponse::Error { message }).await;
    }
  };

  let needs_auth = matches!(
    request,
    StudyRequest::SaveNote { .. }
      | StudyRequest::DeleteNote { .. }
      | StudyRequest::SaveAnnotation { .. }
      | StudyRequest::DeleteAnnotation { .. }
      | StudyRequest::SaveProgress { .. }
      | StudyRequest::SaveReply { .. }
      | StudyRequest::DeleteReply { .. }
      | StudyRequest::LikeReply { .. }
  );
  if needs_auth && user_id.is_empty() {
    return socket::send_json(
      session,
      &StudyResponse::Error { message: "authentication required".into() },
    )
    .await;
  }

  match request {
    // ---- reads: reply directly to the requesting session ----
    StudyRequest::ListNotes => {
      let resp = match study::list_notes(db, user_id).await {
        Ok(notes) => StudyResponse::Notes { notes },
        Err(message) => StudyResponse::Error { message },
      };
      socket::send_json(session, &resp).await
    }
    StudyRequest::ListAnnotations => {
      let resp = match study::list_annotations(db, user_id).await {
        Ok(annotations) => StudyResponse::Annotations { annotations },
        Err(message) => StudyResponse::Error { message },
      };
      socket::send_json(session, &resp).await
    }
    StudyRequest::ListProgress => {
      let resp = match study::list_progress(db, user_id).await {
        Ok(items) => StudyResponse::Progress { items },
        Err(message) => StudyResponse::Error { message },
      };
      socket::send_json(session, &resp).await
    }

    StudyRequest::SubscribeSection { section_path } => {
      *current_section = Some(section_path.clone());
      let resp = match study::list_public_section(db, &section_path).await {
        Ok((annotations, notes, replies)) => StudyResponse::SectionPublic {
          section_path,
          annotations,
          notes,
          replies,
        },
        Err(message) => StudyResponse::Error { message },
      };
      socket::send_json(session, &resp).await
    }

    StudyRequest::SaveNote { note } => {
      match study::save_note(db, user_id, username, note).await {
        Ok((note, was_public)) => {
          let id_hex = note.id.map(|o| o.to_hex()).unwrap_or_default();
          let section = note.section_path.clone();
          let is_public = note.public;
          broadcast(
            app_state,
            user_id,
            StudyResponse::NoteSaved { note: note.clone() },
          );
          if let Some(path) = section {
            if is_public {
              broadcast_section(
                app_state, &path, StudyResponse::NoteSaved { note },
              );
            } else if was_public {
              broadcast_section(
                app_state, &path, StudyResponse::NoteDeleted { id: id_hex },
              );
            }
          }
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::DeleteNote { id } => {
      match study::delete_note(db, user_id, &id).await {
        Ok(meta) => {
          broadcast(
            app_state, user_id, StudyResponse::NoteDeleted { id: id.clone() },
          );
          if let Some((true, Some(path))) = meta {
            broadcast_section(
              app_state, &path, StudyResponse::NoteDeleted { id },
            );
          }
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::SaveAnnotation { annotation } => {
      match study::save_annotation(db, user_id, username, annotation).await {
        Ok((annotation, was_public)) => {
          let id_hex = annotation.id.map(|o| o.to_hex()).unwrap_or_default();
          let section = annotation.section_path.clone();
          let is_public = annotation.public;
          broadcast(
            app_state,
            user_id,
            StudyResponse::AnnotationSaved { annotation: annotation.clone() },
          );
          if is_public {
            broadcast_section(
              app_state, &section,
              StudyResponse::AnnotationSaved { annotation },
            );
          } else if was_public {
            broadcast_section(
              app_state, &section,
              StudyResponse::AnnotationDeleted { id: id_hex },
            );
          }
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::DeleteAnnotation { id } => {
      match study::delete_annotation(db, user_id, &id).await {
        Ok(meta) => {
          broadcast(
            app_state, user_id,
            StudyResponse::AnnotationDeleted { id: id.clone() },
          );
          if let Some((true, path)) = meta {
            broadcast_section(
              app_state, &path, StudyResponse::AnnotationDeleted { id },
            );
          }
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::SaveProgress { progress } => {
      match study::save_progress(db, user_id, progress).await {
        Ok(item) => {
          broadcast(app_state, user_id, StudyResponse::ProgressSaved { item });
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }

    StudyRequest::SaveReply { reply } => {
      match study::save_reply(db, user_id, username, reply).await {
        Ok(reply) => {
          let path = reply.section_path.clone();
          broadcast_section(
            app_state, &path, StudyResponse::ReplySaved { reply },
          );
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::DeleteReply { id } => {
      match study::delete_reply(db, user_id, &id).await {
        Ok(section_path) => {
          broadcast_section(
            app_state, &section_path,
            StudyResponse::ReplyDeleted {
              id,
              section_path: section_path.clone(),
            },
          );
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::LikeReply { id } => {
      match study::like_reply(db, user_id, &id).await {
        Ok(reply) => {
          let path = reply.section_path.clone();
          broadcast_section(
            app_state, &path, StudyResponse::ReplySaved { reply },
          );
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
  }
}
