pub mod views;

use actix_web::{get, Responder};
use serde::{Deserialize, Serialize};
pub use views::*;

// path json struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
  pub target_path: String,
  pub request_path: String,
}

// #[get("/views")]
// async fn get_views(request_path) -> impl Responder {

// }
