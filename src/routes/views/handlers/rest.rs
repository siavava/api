//! REST handlers for view-count endpoints.

use crate::{AppState, all_views, models::views::PageViews, views};

use actix_web::{
  Error as ActixError, HttpResponse, delete, get, post,
  web::{Data, Json, Query},
};
use mongodb::Client;
use serde::Deserialize;

/// Query parameters for `GET /views/` and `DELETE /views/`.
#[derive(Deserialize, Debug)]
pub struct PageViewRequestData {
  /// The route whose views are being requested/deleted.
  pub requested: Option<String>,
  /// The viewer's current page — used to decide whether to increment.
  pub location: Option<String>,
}

/// Request body for `POST /views/` — either a single record or a batch.
#[derive(Deserialize, Debug)]
pub enum PageViewPostData {
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
#[get("/")]
pub async fn get_views(
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
#[delete("/")]
pub async fn delete_views(
  app_state: Data<AppState>,
  request_data: Json<PageViewRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;
  let request_data = request_data.into_inner();
  match request_data.requested {
    Some(requested_str) => {
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
#[post("/")]
pub async fn insert_views(
  client: Data<Client>,
  request_data: Json<PageViewPostData>,
) -> Result<HttpResponse, ActixError> {
  match request_data.into_inner() {
    PageViewPostData::Single(item) => {
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
