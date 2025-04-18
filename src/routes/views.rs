use actix_web::{
  delete, get, post,
  web::{Data, Json, Path, Query},
  App, Error as ActixError, HttpResponse, Responder,
};
use mongodb::Client;
use serde::Deserialize;

use crate::{all_views, views};
use crate::{controllers::views, models::views::PageViews, AppState};

use crate::controllers::PageEventsBroadcaster;

// function to inject routes
pub fn inject_routes(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(get_views);
  cfg.service(delete_views);
  cfg.service(insert_views);
  cfg.service(event_stream);
  cfg.service(broadcast_msg);
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

#[get("/views")]
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
#[delete("/views")]
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

#[post("/views")]
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

#[get("/page-events")]
async fn event_stream(app_state: Data<AppState>) -> impl Responder {
  let broadcaster = &app_state.events_handler;
  broadcaster.new_client().await
}

#[post("/broadcast/{msg}")]
async fn broadcast_msg(app_state: Data<AppState>, msg: Path<String>) -> impl Responder {
  let broadcaster = &app_state.events_handler;
  let msg = msg.into_inner();
  broadcaster.broadcast(&msg).await;
  HttpResponse::Ok().body("msg sent")
}
