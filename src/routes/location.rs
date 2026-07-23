//! # Location Route
//!
//! REST endpoint for reading and updating the tracked location.
//!
//! Exposes `GET /location/` which either records a new location (when both
//! `city` and `state` query params are provided) or reads the current
//! last-known location, and `GET /location/history/` which lists the
//! visitor location history.

use crate::{
  AppState, controllers::location as location_controller, location,
  models::location::LocationEvent,
};

use actix_web::{
  Error as ActixError, HttpResponse, get,
  web::{Data, Query, scope},
};
use mongodb::bson::doc;
use serde::Deserialize;

/// Registers the `/location/` endpoint.
///
/// # Arguments
///
/// * `cfg` — The Actix-Web service config to register routes on.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(
    scope("/location")
      .service(get_location_history)
      .service(get_location),
  );
}

/// Query parameters for `GET /location/`.
#[derive(Deserialize, Debug)]
struct LocationRequestData {
  /// City name to record (optional).
  city: Option<String>,
  /// State/region to record (optional).
  state: Option<String>,
  /// Site namespace to attribute the visit to (optional, e.g. `<p>`).
  ns: Option<String>,
  /// Latitude in degrees (optional, recorded with `lon`).
  lat: Option<f64>,
  /// Longitude in degrees (optional, recorded with `lat`).
  lon: Option<f64>,
}

/// `GET /location/` — reads or updates the tracked location.
///
/// # Behavior
///
/// * **With both `city` and `state` query params** — records the new
///   location (updating both the "last known" document and the history log)
///   and returns the **previous** location as JSON.
/// * **Without both** — returns the current last-known location without
///   modifying it.
///
/// # Example Request
///
/// ```text
/// GET /location/?city=San+Francisco&state=California
/// ```
///
/// # Example Response
///
/// ```json
/// { "city": "New York", "state": "New York" }
/// ```
///
/// (Returns the **previous** location when updating, or the current
/// location when reading.)
///
/// # Returns
///
/// `200 OK` with a JSON [`LocationData`](crate::models::location::LocationData)
/// body.
#[get("/")]
async fn get_location(
  app_state: Data<AppState>,
  request_data: Query<LocationRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;

  let LocationRequestData { city, state, ns, lat, lon } =
    request_data.into_inner();

  match (city, state) {
    (Some(city_str), Some(state_str)) => {
      let (previous, entry) = location_controller::get_last_and_update(
        db_client,
        &city_str,
        &state_str,
        ns.as_deref(),
        lat.zip(lon),
      )
      .await
      .unwrap_or_default();
      if let Some(ns) = ns.as_deref() {
        let _ = crate::controllers::events::record_event(
          db_client,
          ns,
          "visit",
          &format!("{city_str}, {state_str}"),
        )
        .await;
      }
      let _ = app_state.location_events.send(LocationEvent {
        entry,
        namespace: ns,
      });
      Ok(HttpResponse::Ok().json(previous))
    }
    _ => {
      let res = location!(eval & db_client);
      Ok(HttpResponse::Ok().json(res))
    }
  }
}

/// Query parameters for `GET /location/history/`.
#[derive(Deserialize, Debug)]
struct HistoryRequestData {
  /// Site namespace to filter by (optional, e.g. `<p>`).
  ns: Option<String>,
}

/// `GET /location/history/` — lists the visitor location history.
///
/// # Behavior
///
/// Returns every recorded city+state pair with its visit count and the
/// time of its most recent visit, sorted by visit count descending.
///
/// # Example Response
///
/// ```json
/// [
///   { "city": "New York", "state": "New York", "count": 42, "last_visit_ms": 1758000000000 }
/// ]
/// ```
///
/// # Returns
///
/// `200 OK` with a JSON array of
/// [`LocationHistoryEntry`](crate::models::location::LocationHistoryEntry),
/// or `500 Internal Server Error` if the query fails.
///
/// Accepts an optional `ns` query parameter restricting results to one
/// site namespace (e.g. `?ns=<p>`).
#[get("/history/")]
async fn get_location_history(
  app_state: Data<AppState>,
  request_data: Query<HistoryRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;
  let HistoryRequestData { ns } = request_data.into_inner();

  match location_controller::get_location_history(db_client, ns.as_deref())
    .await
  {
    Ok(entries) => Ok(HttpResponse::Ok().json(entries)),
    Err(err) => {
      tracing::error!("failed to read location history: {err}");
      Ok(HttpResponse::InternalServerError().json("Error"))
    }
  }
}
