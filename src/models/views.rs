/// Views model
///
/// This model is used to store the views of the routes
///
use serde::{Deserialize, Serialize};

use mongodb::bson::doc;

/// Views struct
///
/// This struct is used to store the views of the routes
///
/// It tracks the route and the count of the views
#[derive(Serialize, Deserialize)]
pub struct PageViews {
  pub route: String,
  pub count: u64,
}

// impl debug for Views
impl std::fmt::Debug for PageViews {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Views {{ route: {}, count: {} }}",
      self.route, self.count
    )
  }
}

// implement Default for Views
impl Default for PageViews {
  fn default() -> Self {
    PageViews {
      route: "".into(),
      count: 0,
    }
  }
}
