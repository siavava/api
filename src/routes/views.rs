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

use crate::{AppState, all_views, models::views::PageViews, views};

use actix_web::{
  Error as ActixError, HttpResponse, Responder, delete, get, post,
  web::{Data, Html, Json, Query, scope},
};
use mongodb::{Client, bson::doc};
use serde::Deserialize;

/// Registers all `/views/` endpoints (GET, POST, DELETE, and SSE watch).
///
/// # Arguments
///
/// * `cfg` — The Actix-Web service config to register routes on.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(
    scope("/views")
      .service(get_views)
      .service(delete_views)
      .service(insert_views)
      .service(watch_views)
      .service(watch_views_test),
  );
}

/// Query parameters for `GET /views/` and `DELETE /views/`.
#[derive(Deserialize, Debug)]
struct PageViewRequestData {
  /// The route whose views are being requested/deleted.
  requested: Option<String>,
  /// The viewer's current page — used to decide whether to increment.
  location: Option<String>,
}

/// Request body for `POST /views/` — either a single record or a batch.
#[derive(Deserialize, Debug)]
enum PageViewPostData {
  /// Upsert a single route's view count.
  Single(PageViews),
  /// Upsert multiple routes' view counts at once.
  Multiple(Vec<PageViews>),
}

/// `GET /views/` — returns view counts.
///
/// # Behavior
///
/// * **Both `requested` and `location` provided** — returns views for
///   `requested`, incrementing the count only if `requested == location`.
/// * **Neither provided** — returns all view counts sorted by count
///   descending.
/// * **Only one provided** — returns `400 Bad Request`.
///
/// # Example Request (single route)
///
/// ```text
/// GET /views/?requested=/blog/post-1&location=/blog/post-1
/// ```
///
/// # Example Response (single route)
///
/// ```json
/// { "route": "/blog/post-1", "count": 42 }
/// ```
///
/// # Example Response (all routes)
///
/// ```json
/// [
///   { "route": "/blog/post-1", "count": 42 },
///   { "route": "/blog/post-2", "count": 17 }
/// ]
/// ```
///
/// # Returns
///
/// * `200 OK` with a JSON [`PageViews`] or `Vec<PageViews>` body.
/// * `400 Bad Request` if only one of `requested`/`location` is provided.
#[get("/")]
async fn get_views(
  app_state: Data<AppState>,
  request_data: Query<PageViewRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;

  let PageViewRequestData {
    requested,
    location,
  } = request_data.into_inner();

  match (requested, location) {
    (Some(requested_str), Some(location_str)) => {
      let res = views![&db_client, &requested_str, &location_str];
      Ok(HttpResponse::Ok().json(res))
    }
    (None, None) => {
      let res = all_views![&db_client];
      Ok(HttpResponse::Ok().json(res))
    }
    _ => Ok(
      HttpResponse::BadRequest()
        .json("Invalid Request: You must provide neither or both of {target,request}_route"),
    ),
  }
}

/// `DELETE /views/` — deletes the view-count document for the given route.
///
/// # Example Request
///
/// ```json
/// { "requested": "/blog/post-1" }
/// ```
///
/// # Example Response
///
/// ```json
/// "Deleted"
/// ```
///
/// # Returns
///
/// * `200 OK` with `"Deleted"` on success.
/// * `400 Bad Request` if `requested` is missing.
/// * `500 Internal Server Error` on DB failure.
#[delete("/")]
async fn delete_views(
  app_state: Data<AppState>,
  request_data: Json<PageViewRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;
  let request_data = request_data.into_inner();
  match request_data.requested {
    Some(requested_str) => {
      // delete views
      let res = views::delete_views(db_client, &requested_str).await;
      match res {
        Ok(_) => Ok(HttpResponse::Ok().json("Deleted")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error")),
      }
    }
    None => Ok(HttpResponse::BadRequest().json("Invalid Request: You must provide `requested`")),
  }
}

/// `POST /views/` — upserts one or more view-count records.
///
/// # Example Request (single)
///
/// ```json
/// { "Single": { "route": "/blog/post-1", "count": 42 } }
/// ```
///
/// # Example Request (batch)
///
/// ```json
/// { "Multiple": [
///   { "route": "/blog/post-1", "count": 42 },
///   { "route": "/blog/post-2", "count": 17 }
/// ] }
/// ```
///
/// # Example Response
///
/// ```json
/// "Inserted"
/// ```
///
/// # Returns
///
/// * `200 OK` with `"Inserted"` on success.
/// * `500 Internal Server Error` on DB failure.
#[post("/")]
async fn insert_views(
  client: Data<Client>,
  request_data: Json<PageViewPostData>,
) -> Result<HttpResponse, ActixError> {
  match request_data.into_inner() {
    PageViewPostData::Single(item) => {
      // if exists, update
      // if not, insert
      // let res = views::get_views(&client, &item.route, ).await;
      let res = views::insert_view(&client, item).await;
      match res {
        Ok(_) => Ok(HttpResponse::Ok().json("Inserted")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error")),
      }
    }
    PageViewPostData::Multiple(data) => {
      let res = views::insert_views(&client, data).await;
      match res {
        Ok(_) => Ok(HttpResponse::Ok().json("Inserted")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error")),
      }
    }
  }
}

/// `GET /views/watch/` — SSE endpoint for real-time view-count updates.
///
/// # Query Parameters
///
/// * `requested` (optional) — Filter events to a specific route.
///   If omitted, receives updates for all routes (wildcard filter).
///
/// # Example Request
///
/// ```text
/// GET /views/watch/?requested=/blog/post-1
/// ```
///
/// # Example SSE Events
///
/// ```text
/// event: connected
/// data: connected
///
/// event: update
/// data: {"route":"/blog/post-1","count":43}
///
/// event: count
/// data: {"count":3}
/// ```
///
/// # Returns
///
/// An SSE stream (`text/event-stream`). The connection stays open
/// indefinitely, pushing `update` events when view counts change and
/// `count` events when the number of connected listeners changes.
#[get("/watch/")]
async fn watch_views(
  app_state: Data<AppState>,
  Query(PageViewRequestData {
    requested,
    location: _,
  }): Query<PageViewRequestData>,
) -> impl Responder {
  let filter = {
    match requested {
      // if it's a valid route, return with that route
      Some(route) => PageViews::with(route),

      // if it's empty, return default
      // NOTE: default will pass filter
      None => PageViews::default(),
    }
  };

  let broadcaster = &app_state.view_events_handler;
  broadcaster.new_client(filter).await
}

/// `GET /views/watch/test/` — serves a static HTML page for testing the
/// SSE watch endpoint.
///
/// # Returns
///
/// `200 OK` with an HTML page that connects to the SSE watch endpoint
/// and displays live view-count updates.
#[get("/watch/test/")]
async fn watch_views_test() -> impl Responder {
  Html::new(include_str!("../static/watch-views.html"))
}
