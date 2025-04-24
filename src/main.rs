use actix_cors::Cors;
use actix_web::{
  App, HttpServer, Responder, get,
  http::{self},
  middleware::{Logger, NormalizePath, TrailingSlash},
  web,
};
use dotenv::dotenv;
use mongodb::{
  Client,
  options::{ClientOptions, ServerApi, ServerApiVersion},
};
use server::{AppState, app_state, routes::views};
use std::{env, io::Result};
use tracing::{error, info};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
  format!("Hello {name}!")
}

#[get("/")]
async fn health_check() -> impl Responder {
  "Ok."
}

#[actix_web::main]
async fn main() -> Result<()> {
  const DEFAULT_PORT: u16 = 3000;

  dotenv().ok();
  tracing_subscriber::fmt::init();

  let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not set in environment variables!");

  let port = {
    let res = env::var("PORT");
    match res {
      Ok(value) => value.parse::<u16>().unwrap_or_else(|err| {
        error!("ERROR PARSING PROVIDED PORT '{value}': {err}");
        error!("PLEASE MAKE SURE IT IS A VALID INTEGER.");
        error!("DEFAULTING TO PORT {DEFAULT_PORT}");
        DEFAULT_PORT
      }),
      Err(e) => {
        error!("{e}");
        error!("PORT NOT SET IN ENVIRONMENT VARIABLES.");
        error!("DEFAULTING TO PORT {DEFAULT_PORT}");
        DEFAULT_PORT
      }
    }
  };

  let mut client_options = ClientOptions::parse(mongodb_uri).await.unwrap();

  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  let db_client = Client::with_options(client_options.clone()).unwrap();

  let app_state = app_state!(db_client.clone());

  info!("STARTING APP");
  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .wrap(NormalizePath::new(TrailingSlash::Always))
      .wrap(
        Cors::default()
          .allow_any_origin() // <--- this
          // .allowed_origin_fn(verify_cors)
          .allowed_methods(vec!["GET", "PUT", "POST", "DELETE"])
          .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
          .allowed_header(http::header::CONTENT_TYPE)
          .max_age(3600),
      )
      .app_data(web::Data::<AppState>::new(app_state.clone()))
      .service(health_check)
      .service(greet)
      .configure(views::register)
  })
  // .bind(("127.0.0.1", port))?
  .bind(("0.0.0.0", port))?
  .run()
  .await
}

// /**
//   CORS verification function

//   ### Example
//   ```rust
//   let origin = HeaderValue::from_static("http://localhost:3000");
//   let req_head = RequestHead::default();
//   let result = verify_cors(&origin, &req_head);
//   assert_eq!(result, true);
//   ```
// */
// fn verify_cors(origin: &HeaderValue, _req_head: &RequestHead) -> bool {
//   let allowed_origins = ["amittai.studio", "amittai.space", "localhost"];
//   let origin = origin.to_str();

//   matches!(origin, Ok(val) if allowed_origins
//     .iter()
//     .any(|allowed_origin| val.contains(allowed_origin)))
// }
