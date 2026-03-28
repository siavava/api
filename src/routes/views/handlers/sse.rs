//! SSE handlers for real-time view-count updates.

use super::rest::PageViewRequestData;
use crate::{AppState, models::views::PageViews};

use actix_web::{
  Responder, get,
  web::{Data, Html, Query},
};

/// `GET /views/watch/` — SSE endpoint for real-time view-count updates.
#[get("/watch/")]
pub async fn watch_views(
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
pub async fn watch_views_test() -> impl Responder {
  Html::new(include_str!("../../../static/watch-views.html"))
}
