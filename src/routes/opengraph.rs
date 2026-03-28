//! # OpenGraph Route
//!
//! REST endpoint for fetching OpenGraph metadata from a given URL.
//!
//! # Endpoints
//!
//! | Method | Path            | Response | Description                        |
//! |--------|-----------------|----------|------------------------------------|
//! | GET    | `/opengraph/`   | JSON     | OpenGraph metadata for the given URL. |

use crate::controllers::opengraph;

use actix_web::{
  HttpResponse, get,
  web::{Query, scope},
};
use serde::Deserialize;

/// Registers the `/opengraph/` endpoint.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(scope("/opengraph").service(get_opengraph));
}

/// Query parameters for `GET /opengraph/`.
#[derive(Deserialize, Debug)]
struct OpenGraphRequest {
  /// The URL to fetch OpenGraph data from.
  url: String,
}

/// `GET /opengraph/?url=...` — fetches and returns OpenGraph metadata for the given URL.
#[get("/")]
async fn get_opengraph(query: Query<OpenGraphRequest>) -> HttpResponse {
  let target_url = &query.url;

  match opengraph::fetch_opengraph(target_url).await {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
      "error": e
    })),
  }
}
