use actix_web::{
  get,
  web::{Data, Json},
  Responder,
};
use mongodb::Client;
use serde::Deserialize;

use crate::views;

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

#[get("/v")]
async fn test_views(client: Data<Client>) -> impl Responder {
  // this

  let res = views![&client, "/test", "/test"];

  format!("{:?}", res)
}

#[get("/views")]
async fn get_views(
  client: Data<Client>,
  request_data: Json<PageViewRequestData>,
) -> impl Responder {
  let res = views![
    &client,
    &request_data.target_route,
    &request_data.request_route
  ];

  format!("{:?}", res)
}
