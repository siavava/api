use crate::{AppState, all_views, models::views::PageViews, views};

use actix_web::{
  Error as ActixError, HttpResponse, Responder, delete, get, post,
  web::{Data, Html, Json, Query, scope},
};
use mongodb::{Client, bson::doc};
use serde::Deserialize;

// function to inject routes
pub fn inject_routes(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(
    scope("/views")
      .service(get_views)
      .service(delete_views)
      .service(insert_views)
      .service(watch_views)
      .service(watch_views_test),
  );
}

#[derive(Deserialize, Debug)]
struct PageViewRequestData {
  target_route: Option<String>,
  request_route: Option<String>,
}

#[derive(Deserialize, Debug)]
enum PageViewPostData {
  Single(PageViews),
  Multiple(Vec<PageViews>),
}

#[get("/")]
async fn get_views(
  app_state: Data<AppState>,
  request_data: Query<PageViewRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;

  let PageViewRequestData {
    target_route,
    request_route,
  } = request_data.into_inner();

  match (target_route, request_route) {
    (Some(target_str), Some(request_str)) => {
      let res = views![&db_client, &target_str, &request_str];
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

// delete views
#[delete("/")]
async fn delete_views(
  app_state: Data<AppState>,
  request_data: Json<PageViewRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;
  let request_data = request_data.into_inner();
  match request_data.target_route {
    Some(target_route) => {
      // delete views
      let res = views::delete_views(db_client, &target_route).await;
      match res {
        Ok(_) => Ok(HttpResponse::Ok().json("Deleted")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error")),
      }
    }
    None => Ok(HttpResponse::BadRequest().json("Invalid Request: You must provide target_route")),
  }
}

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

#[get("/watch/")]
async fn watch_views(
  app_state: Data<AppState>,
  request_data: Query<PageViewRequestData>,
) -> impl Responder {
  let filter = {
    let Query(PageViewRequestData {
      target_route,
      request_route: _,
    }) = request_data;

    match target_route {
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

#[get("/watch/test/")]
async fn watch_views_test() -> impl Responder {
  Html::new(include_str!("../index.html").to_owned())
}
