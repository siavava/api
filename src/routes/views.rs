use actix_web::{
  get,
  web::{Data, Json},
  Error as ActixError, HttpResponse,
};
use mongodb::Client;
use serde::Deserialize;

use crate::{all_views, views};

// function to inject routes
pub fn views_routes(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(test_views);
  cfg.service(get_views);
}

#[derive(Deserialize)]
struct PageViewRequestData {
  target_route: String,
  request_route: String,
}

#[get("/views/all")]
async fn test_views(client: Data<Client>) -> Result<HttpResponse, ActixError> {
  let res = all_views![&client];
  Ok(HttpResponse::Ok().json(res))
}

#[get("/views")]
async fn get_views(
  client: Data<Client>,
  request_data: Json<PageViewRequestData>,
) -> Result<HttpResponse, ActixError> {
  let res = views![
    &client,
    &request_data.target_route,
    &request_data.request_route
  ];
  Ok(HttpResponse::Ok().json(res))
}
