//! # Events Route
//!
//! REST endpoint serving the rolling site-event log to metrics
//! dashboards.

use crate::{AppState, controllers::events as events_controller};

use actix_web::{
  Error as ActixError, HttpResponse, get,
  web::{Data, Query, scope},
};
use serde::Deserialize;

/// Registers the `/events/` endpoint.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(scope("/events").service(get_events));
}

/// Query parameters for `GET /events/`.
#[derive(Deserialize, Debug)]
struct EventsRequestData {
  /// The site namespace to read events for (e.g. `<p>`).
  ns: String,
  /// Maximum events to return. Defaults to 50.
  limit: Option<i64>,
}

/// `GET /events/` — the namespace's most recent site events.
///
/// # Example Response
///
/// ```json
/// [{ "kind": "view", "label": "/projects/x", "ts_ms": 1784800000000 }]
/// ```
#[get("/")]
async fn get_events(
  app_state: Data<AppState>,
  request_data: Query<EventsRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;
  let EventsRequestData { ns, limit } = request_data.into_inner();
  let limit = limit.unwrap_or(50).clamp(1, 200);

  match events_controller::get_events(db_client, &ns, limit).await {
    Ok(events) => Ok(HttpResponse::Ok().json(events)),
    Err(err) => {
      tracing::error!("failed to read site events: {err}");
      Ok(HttpResponse::InternalServerError().json("Error"))
    }
  }
}
