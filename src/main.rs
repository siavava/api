use actix_cors::Cors;
use actix_web::{
  App, HttpServer, Responder, get,
  http::{self},
  middleware::{Logger, NormalizePath, TrailingSlash},
  web::{self, Html},
};
use dotenv::dotenv;
use mongodb::{
  Client,
  options::{ClientOptions, ServerApi, ServerApiVersion},
};
use rand;
use server::{AppState, app_state, routes};
use std::{env, io::Result};
use tracing::{error, info};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
  format!("Hello {name}!")
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

  let db_client = Client::with_options(client_options).unwrap();

  let app_state = app_state!(db_client);

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
      .service(quotidian)
      .service(greet)
      .configure(routes::register)
  })
  // .bind(("127.0.0.1", port))?
  .bind(("0.0.0.0", port))?
  .run()
  .await
}

#[get("/")]
async fn quotidian() -> impl Responder {
  let raw_data = include_str!("static/quotes.json");
  let json: serde_json::Value =
    serde_json::from_str(raw_data).expect("JSON was not well-formatted");

  let quotes = json["quotes"].as_array().expect("Quotes not found in JSON");

  // return random element from quotes
  let random_quote = &quotes[rand::random::<usize>() % quotes.len()];

  let quote_text = random_quote["text"].as_str();
  let quote_author = random_quote["author"].as_str();

  match (quote_text, quote_author) {
    (Some(quote), Some(author)) => {
      // format!("{} <br> - {}", quote, author)
      Html::new(format!(
        r#"
        <body
          style="
            margin: 0; padding: 0; width: 100svw; height: 100svh;
            background-color: #111110; color: #d9d8e1e6;
            display: flex;
            justify-content: center;
            align-items: center;
            font-family: system-ui, sans-serif;
          ">

          <div style="
            display: flex;
            flex-direction: column;
            gap: 2rem;
            align-items: flex-end;
            width: min(90%, 44ch);
            // border: 1px solid red;
            padding: 2rem;
            line-height: 1.5;
          ">
            <div style="width: 100%; text-align: left;">
              {quote}
            </div>
            <div style="width: 100%; text-align: right; display: block;">
              ~ {author}
            </div>
          </div>
        </body>
        "#
      ))
    }
    _ => Html::new("No quote found"),
  }

  // "These violent delights have violent ends."
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
