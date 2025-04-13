use actix_web::{
  delete, get, post,
  web::{Data, Json},
  Error as ActixError, HttpResponse,
};
use mongodb::Client;
use serde::Deserialize;

use crate::{all_views, views};
use crate::{controllers::views, models::views::PageViews};

// function to inject routes
pub fn inject_routes(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(get_views);
  cfg.service(delete_views);
  cfg.service(insert_views);
}

#[derive(Deserialize, Debug)]
struct PageViewRequestData {
  target_route: String,
  request_route: String,
}

#[derive(Deserialize, Debug)]
enum PageViewPostData {
  Single(PageViews),
  Multiple(Vec<PageViews>),
}

#[get("/views")]
async fn get_views(
  client: Data<Client>,
  request_data: Option<Json<PageViewRequestData>>,
) -> Result<HttpResponse, ActixError> {
  match request_data {
    Some(data) => {
      let request_data = data.into_inner();
      let res = views![
        &client,
        &request_data.target_route,
        &request_data.request_route
      ];
      Ok(HttpResponse::Ok().json(res))
    }
    None => {
      let res = all_views![&client];
      Ok(HttpResponse::Ok().json(res))
    }
  }
}

// delete views
#[delete("/views")]
async fn delete_views(
  client: Data<Client>,
  request_data: Json<PageViewRequestData>,
) -> Result<HttpResponse, ActixError> {
  let res = views::delete_views(&client, &request_data.target_route).await;
  match res {
    Ok(_) => Ok(HttpResponse::Ok().json("Deleted")),
    Err(_) => Ok(HttpResponse::InternalServerError().json("Error")),
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
