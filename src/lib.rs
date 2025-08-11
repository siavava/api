use controllers::{EventsBroadcaster, views};
use models::views::PageViews;
use mongodb::Client;
use std::sync::Arc;

// exported mods
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
    let view_events_handler = EventsBroadcaster::<PageViews>::create(views_collection, true);

    AppState {
      db_client,
      view_events_handler,
    }
  }
}

// macro app_state!
#[macro_export]
macro_rules! app_state {
  ($db_client:expr) => {
    $crate::AppState::new($db_client)
  };
}
