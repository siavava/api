//! # `server` — Personal API Server
//!
//! An Actix-Web application backed by MongoDB, providing endpoints for:
//!
//! - **Comments** — real-time blog comment operations over WebSocket.
//! - **Views** — page view counting with REST and SSE live updates.
//! - **Location** — location tracking (last known + visit history).
//! - **Quotes** — quote display via REST and a self-contained HTML page.
//!
//! # Crate Structure
//!
//! | Module                    | Description                                        |
//! |---------------------------|----------------------------------------------------|
//! | [`controllers`]           | Core logic — DB operations and the SSE broadcaster.|
//! | [`models`]                | Data models shared across controllers and routes.  |
//! | [`routes`]                | HTTP endpoint definitions and request handling.    |
//! | [`db`]                    | Database helpers (collection accessor, DB name).   |
//!
//! # Application State
//!
//! [`AppState`] holds the shared MongoDB client and SSE broadcaster.
//! It is cloned into every Actix-Web worker via `web::Data`.
//! Use the [`app_state!`] macro for convenient construction.

use crate::{
  controllers::{playback::SpotifyClient, views},
  models::{
    comments::CommentEvent,
    now::NowEvent,
    views::{PageViews, ViewEvent},
  },
};
use mongodb::Client;
use protocol::sse;
use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::broadcast;

pub mod controllers;
pub mod db;
pub mod models;
pub mod protocol;
pub mod routes;

/// Shared application state, passed to all route handlers via
/// `actix_web::web::Data`.
#[derive(Clone)]
pub struct AppState {
  /// MongoDB client used by all controllers.
  pub db_client: Client,
  /// SSE broadcaster for real-time page view updates.
  /// Watches the `views` collection and pushes changes to connected SSE clients.
  pub view_events_handler: Arc<sse::EventsBroadcaster<PageViews>>,
  /// Broadcast channel for real-time comment events (create, edit, like, delete).
  /// Each WebSocket client subscribes and receives events for its active route.
  pub comment_events: broadcast::Sender<CommentEvent>,
  /// Broadcast channel for real-time view-count events.
  pub view_events: broadcast::Sender<ViewEvent>,
  /// Number of currently connected WebSocket clients.
  pub active_clients: Arc<AtomicUsize>,
  /// Broadcast channel for active-client-count changes.
  /// Sent to ALL clients regardless of active path.
  pub active_count_events: broadcast::Sender<usize>,
  /// Broadcast channel for "now"-slot mutations.
  /// Sent to ALL clients so the dynamic island updates live.
  pub now_events: broadcast::Sender<NowEvent>,
  /// Spotify API client for playback data.
  /// `None` if Spotify credentials are not configured.
  pub spotify: Option<Arc<SpotifyClient>>,
  /// Broadcast channel for study-network mutations (notes, annotations,
  /// progress). Events carry the owning `user_id`; each authenticated study
  /// WebSocket only forwards events matching its own user. Fully isolated
  /// from the blog scopes above.
  pub study_events: broadcast::Sender<crate::models::study::StudyEvent>,
  /// Secret used to sign/verify study auth JWTs. Loaded from `JWT_SECRET`
  /// (falls back to a dev-only default).
  pub jwt_secret: Arc<String>,
}

impl AppState {
  /// Creates a new [`AppState`], initializing broadcasters.
  pub fn new(db_client: Client) -> Self {
    let views_collection = views::get_collection(&db_client);
    let view_events_handler =
      sse::EventsBroadcaster::<PageViews>::create(views_collection, true);
    let (comment_events, _) = broadcast::channel::<CommentEvent>(256);
    let (view_events, _) = broadcast::channel::<ViewEvent>(256);
    let (active_count_events, _) = broadcast::channel::<usize>(256);
    let (now_events, _) = broadcast::channel::<NowEvent>(64);

    let spotify = SpotifyClient::from_env().map(Arc::new);

    let (study_events, _) =
      broadcast::channel::<crate::models::study::StudyEvent>(256);
    let jwt_secret = Arc::new(
      std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-only-insecure-study-secret".to_string()),
    );

    Self {
      db_client,
      view_events_handler,
      comment_events,
      view_events,
      active_clients: Arc::new(AtomicUsize::new(0)),
      active_count_events,
      now_events,
      spotify,
      study_events,
      jwt_secret,
    }
  }
}

/// Convenience macro for constructing [`AppState`] from a MongoDB `Client`.
///
/// # Usage
///
/// ```ignore
/// let state = app_state!(db_client);
/// ```
#[macro_export]
macro_rules! app_state {
  ($db_client:expr) => {
    $crate::AppState::new($db_client)
  };
}
