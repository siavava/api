use mongodb::{
  options::{ClientOptions, ServerApi, ServerApiVersion},
  Client,
};

use std::io::Result;

use dotenv::dotenv;

use wsserver::routes::views::views_routes;

use actix_web::{get, web, App, HttpServer, Responder};

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
  dotenv().ok();
  let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set in .env file");

  let mut client_options = ClientOptions::parse(mongodb_uri).await.unwrap();

  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  let client = Client::with_options(client_options).unwrap();

  println!("STARTING APP");
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(client.clone()))
      .service(health_check)
      .service(greet)
      .configure(views_routes)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
