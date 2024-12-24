/// Views model
///
/// This model is used to store the views of the routes
///
/// # Example
///
/// ```
/// use wserver::models::Views;
///
/// let views = Views {
///  route: "/api".into(),
///  count: 1,
/// };
///
/// println!("views: {:?}", views);
/// ```
///
use serde::{Deserialize, Serialize};

/// Views struct
///
/// This struct is used to store the views of the routes
///
/// It tracks the route and the count of the views
#[derive(Serialize, Deserialize)]
pub struct Views {
  pub route: String,
  pub count: u32,
}

// impl debug for Views
impl std::fmt::Debug for Views {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Views {{ route: {}, count: {} }}",
      self.route, self.count
    )
  }
}

///
/// Macro to create a new Views instance
///
/// # Example
///
/// ```
/// use wserver::views;
///
/// let views = views!["/api", 1];
///
/// println!("views: {:?}", views);
/// ```
///
#[macro_export]
macro_rules! views {
  ($route:expr, $count:expr) => {
    Views {
      route: $route.into(),
      count: $count,
    }
  };
}
