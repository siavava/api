use mongodb::{
  bson::doc,
  error::Result,
  options::{ClientOptions, ServerApi, ServerApiVersion},
  Client,
};

use wsserver::views;

#[tokio::main]
async fn main() -> Result<()> {
  println!("Hello, world!");
  // TODO: do something here.

  let mut client_options =
    ClientOptions::parse("mongodb+srv://siavava:what2three@blog-feed.vleka.mongodb.net/?retryWrites=true&w=majority&appName=blog-feed").await?;

  // Set the server_api field of the client_options object to set the version of the Stable API on the client
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);
  // Get a handle to the cluster
  let client = Client::with_options(client_options)?;
  // Ping the server to see if you can connect to the cluster
  client
    .database("admin")
    .run_command(doc! {"ping": 1})
    .await?;

  println!("Pinged your deployment. You successfully connected to MongoDB! What else?");

  let db = client.database("blog-feed");

  let views = views![&db, "/afternoon"];

  println!("views: {:?}", views);

  Ok(())
}
