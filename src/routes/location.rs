//! # Location Route
//!
//! REST endpoint for reading and updating the tracked location.
//!
//! Exposes `GET /location/` which either records a new location (when both
//! `city` and `state` query params are provided) or reads the current
//! last-known location.

use crate::{AppState, location};

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
  cfg.service(scope("/location").service(get_location));
}

/// Query parameters for `GET /location/`.
#[derive(Deserialize, Debug)]
struct LocationRequestData {
  /// City name to record (optional).
  city: Option<String>,
  /// State/region to record (optional).
  state: Option<String>,
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

  let LocationRequestData { city, state } = request_data.into_inner();

  match (city, state) {
    (Some(city_str), Some(state_str)) => {
      let res = location!(eval & db_client, eval & city_str, eval & state_str);
      Ok(HttpResponse::Ok().json(res))
    }
    _ => {
      let res = location!(eval & db_client);
      Ok(HttpResponse::Ok().json(res))
    }
  }
}
