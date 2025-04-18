use std::sync::Arc;

use controllers::{views, EventsBroadcaster};
use models::views::PageViews;
use mongodb::Client;

// create mod.rs file in models folder
pub mod controllers;
pub mod models;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
  pub db_client: Client,
  pub view_events_handler: Arc<EventsBroadcaster<PageViews>>,
}

impl AppState {
  pub fn new(db_client: Client) -> Self {
    let views_collection = views::get_collection(&db_client);
    let view_events_handler = EventsBroadcaster::<PageViews>::create(views_collection);

    AppState {
      db_client,
      view_events_handler,
    }
  }
}

// macro app_state!
#[macro_export]
macro_rules! app_state {
  ($app_state:expr) => {
    $crate::AppState::new($app_state)
  };
}
