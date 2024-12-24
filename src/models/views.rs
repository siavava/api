use serde::{Deserialize, Serialize};

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

// macro for views
#[macro_export]
macro_rules! views {
  ($route:expr, $count:expr) => {
    Views {
      route: $route.into(),
      count: $count,
    }
  };
}
