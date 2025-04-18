use std::sync::Arc;

use controllers::PageEventsBroadcaster;
use mongodb::Client;

// create mod.rs file in models folder
pub mod controllers;
pub mod models;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
  pub db_client: Client,
  pub events_handler: Arc<PageEventsBroadcaster>,
}
