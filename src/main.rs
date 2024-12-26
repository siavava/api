use mongodb::{
  error::Result,
  options::{ClientOptions, ServerApi, ServerApiVersion},
  Client,
};

use dotenv::dotenv;

use wsserver::views;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();
  let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set in .env file");

  let mut client_options = ClientOptions::parse(mongodb_uri).await?;

  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  let client = Client::with_options(client_options)?;
  let db = client.database("blog-feed");

  // test connection
  let views = views![&db, "/afternoon"];

  println!("views: {:?}", views);

  Ok(())
}
