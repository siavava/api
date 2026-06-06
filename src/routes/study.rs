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
    StudyRequest, StudyResponse,
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

/// `GET /study/connect/?token=JWT` — authenticated study WebSocket.
#[get("/connect/")]
async fn connect_ws(
  req: HttpRequest,
  stream: actix_web::web::Payload,
  query: Query<ConnectQuery>,
  app_state: Data<AppState>,
) -> Result<HttpResponse, ActixError> {
  let token = query.token.clone().unwrap_or_default();
  let claims = match study::verify_token(&app_state.jwt_secret, &token) {
    Ok(c) => c,
    Err(_) => return Ok(HttpResponse::Unauthorized().json(err("invalid token"))),
  };

  let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;
  actix_web::rt::spawn(study_event_loop(
    session,
    msg_stream,
    app_state.into_inner().as_ref().clone(),
    claims.sub,
  ));
  Ok(response)
}

/// Per-client event loop. Multiplexes incoming requests with this user's
/// mutation broadcasts (so a note saved in one tab appears live in another).
async fn study_event_loop(
  mut session: Session,
  mut msg_stream: actix_ws::MessageStream,
  app_state: AppState,
  user_id: String,
) {
  let mut events = app_state.study_events.subscribe();

  loop {
    tokio::select! {
      ws_msg = msg_stream.next() => {
        let Some(Ok(msg)) = ws_msg else { break };
        match msg {
          Message::Text(text) => {
            if !handle_text(&app_state, &user_id, &text, &mut session).await {
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
        let Ok(StudyEvent { user_id: owner, response }) = event else { continue };
        if owner != user_id { continue; }
        if !socket::send_json(&mut session, &response).await { break; }
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

/// Handles one text frame. Returns `false` if the connection should close.
async fn handle_text(
  app_state: &AppState,
  user_id: &str,
  text: &str,
  session: &mut Session,
) -> bool {
  let db = &app_state.db_client;
  let request = match StudyRequest::parse(text) {
    Ok(r) => r,
    Err(message) => {
      return socket::send_json(session, &StudyResponse::Error { message }).await;
    }
  };

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

    // ---- mutations: broadcast so all the user's sessions stay in sync ----
    StudyRequest::SaveNote { note } => {
      match study::save_note(db, user_id, note).await {
        Ok(note) => {
          broadcast(app_state, user_id, StudyResponse::NoteSaved { note });
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::DeleteNote { id } => {
      match study::delete_note(db, user_id, &id).await {
        Ok(()) => {
          broadcast(app_state, user_id, StudyResponse::NoteDeleted { id });
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::SaveAnnotation { annotation } => {
      match study::save_annotation(db, user_id, annotation).await {
        Ok(annotation) => {
          broadcast(
            app_state,
            user_id,
            StudyResponse::AnnotationSaved { annotation },
          );
          true
        }
        Err(message) => {
          socket::send_json(session, &StudyResponse::Error { message }).await
        }
      }
    }
    StudyRequest::DeleteAnnotation { id } => {
      match study::delete_annotation(db, user_id, &id).await {
        Ok(()) => {
          broadcast(app_state, user_id, StudyResponse::AnnotationDeleted { id });
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
  }
}
