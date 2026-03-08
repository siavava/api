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

use controllers::{EventsBroadcaster, views};
use models::views::PageViews;
use mongodb::Client;
use std::sync::Arc;

pub mod controllers;
pub mod db;
pub mod models;
pub mod routes;

/// Shared application state, passed to all route handlers via
/// `actix_web::web::Data`.
///
/// # Fields
///
/// * `db_client` — MongoDB client used by all controllers.
/// * `view_events_handler` — SSE broadcaster for real-time page view
///   updates. Watches the `views` collection and pushes changes to
///   connected clients.
#[derive(Clone)]
pub struct AppState {
  /// MongoDB client used by all controllers.
  pub db_client: Client,
  /// SSE broadcaster for real-time page view updates.
  /// Watches the `views` collection and pushes changes to connected clients.
  pub view_events_handler: Arc<EventsBroadcaster<PageViews>>,
}

impl AppState {
  /// Creates a new [`AppState`], initializing the views SSE broadcaster.
  ///
  /// # Arguments
  ///
  /// * `db_client` — A connected MongoDB client.
  ///
  /// # Side Effects
  ///
  /// Spawns two background Actix-rt tasks:
  /// 1. A change-stream listener on the `views` collection.
  /// 2. A heartbeat ping loop for stale-client detection.
  pub fn new(db_client: Client) -> Self {
    let views_collection = views::get_collection(&db_client);
    let view_events_handler = EventsBroadcaster::<PageViews>::create(views_collection, true);

    Self {
      db_client,
      view_events_handler,
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
