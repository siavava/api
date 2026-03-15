//! # Views Route
//!
//! REST and SSE endpoints for page view counts.
//!
//! # Endpoints
//!
//! | Method | Path                | Description                                    |
//! |--------|---------------------|------------------------------------------------|
//! | GET    | `/views/`           | Read view counts (single or all).              |
//! | POST   | `/views/`           | Upsert one or more view-count records.         |
//! | DELETE | `/views/`           | Delete a route's view-count document.          |
//! | GET    | `/views/watch/`     | SSE stream of real-time view-count updates.    |
//! | GET    | `/views/watch/test/`| Static HTML page for testing the SSE endpoint. |

mod handlers;

use crate::{AppState, models::views::PageViews};
use handlers::PageViewRequestData;

use actix_web::{
  Responder, get,
  web::{Data, Html, Query, scope},
};

/// Registers all `/views/` endpoints (GET, POST, DELETE, and SSE watch).
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(
    scope("/views")
      .service(handlers::get_views)
      .service(handlers::delete_views)
      .service(handlers::insert_views)
      .service(watch_views)
      .service(watch_views_test),
  );
}

/// `GET /views/watch/` — SSE endpoint for real-time view-count updates.
#[get("/watch/")]
async fn watch_views(
  app_state: Data<AppState>,
  Query(PageViewRequestData {
    requested,
    location: _,
  }): Query<PageViewRequestData>,
) -> impl Responder {
  let filter = match requested {
    Some(route) => PageViews::with(route),
    None => PageViews::default(),
  };

  let broadcaster = &app_state.view_events_handler;
  broadcaster.new_client(filter).await
}

/// `GET /views/watch/test/` — serves a static HTML page for testing the
/// SSE watch endpoint.
#[get("/watch/test/")]
async fn watch_views_test() -> impl Responder {
  Html::new(include_str!("../../static/watch-views.html"))
}
