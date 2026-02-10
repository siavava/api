use crate::{AppState, location};

use actix_web::{
  Error as ActixError, HttpResponse, get,
  web::{Data, Query, scope},
};
use log::info;
use mongodb::bson::doc;
use serde::Deserialize;

// function to inject routes
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(scope("/location").service(get_location));
}

#[derive(Deserialize, Debug)]
struct LocationRequestData {
  city: Option<String>,
  state: Option<String>,
}

#[get("/")]
async fn get_location(
  app_state: Data<AppState>,
  request_data: Query<LocationRequestData>,
) -> Result<HttpResponse, ActixError> {
  let db_client = &app_state.db_client;

  let LocationRequestData { city, state } = request_data.into_inner();

  match (city, state) {
    (Some(city_str), Some(state_str)) => {
      info!("fetching city: {}, state: {}", city_str, state_str);
      let res = location!(eval & db_client, eval & city_str, eval & state_str);
      info!("res (1): {:?}", res);
      Ok(HttpResponse::Ok().json(res))
    }
    _ => {
      info!("fetching last known location without updating");
      let res = location!(eval & db_client);
      info!("res (2): {:?}", res);
      Ok(HttpResponse::Ok().json(res))
    }
  }
}
