use mongodb::{
  options::{ClientOptions, ServerApi, ServerApiVersion},
  Client,
};

use std::{env, io::Result};

use dotenv::dotenv;

use wsserver::routes::views;

use actix_web::{get, web, App, HttpServer, Responder};

// cors
use actix_cors::Cors;

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

  let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not set in environment variables!");

  let port = {
    let res = env::var("PORT");
    match res {
      Ok(value) => value.parse::<u16>().unwrap_or_else(|err| {
        println!("ERROR PARSING PROVIDED PORT '{value}': {err}");
        println!("PLEASE MAKE SURE IT IS A VALID INTEGER.");
        println!("DEFAULTING TO PORT {DEFAULT_PORT}");
        DEFAULT_PORT
      }),
      Err(e) => {
        println!("{e}");
        println!("PORT NOT SET IN ENVIRONMENT VARIABLES.");
        println!("DEFAULTING TO PORT {DEFAULT_PORT}");
        DEFAULT_PORT
      }
    }
  };

  let mut client_options = ClientOptions::parse(mongodb_uri).await.unwrap();

  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  let client = Client::with_options(client_options).unwrap();

  println!("STARTING APP");
  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::default()
          .allowed_origin("https://amittai.studio")
          .allowed_origin("https://amittai.space")
          .allowed_methods(vec!["GET", "PUT", "POST", "DELETE"])
          .max_age(3600),
      )
      .app_data(web::Data::new(client.clone()))
      .service(health_check)
      .service(greet)
      .configure(views::inject_routes)
  })
  // .bind(("127.0.0.1", port))?
  .bind(("0.0.0.0", port))?
  .run()
  .await
}
