/// Views model
///
/// This model is used to store the views of the routes
///
use bytestring::ByteString;
use serde::{Deserialize, Serialize};

use mongodb::bson::doc;

/// Views struct
///
/// This struct is used to store the views of the routes
///
/// It tracks the route and the count of the views
#[derive(Serialize, Deserialize, Clone)]
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

// convert PagViews to bytestring
impl std::convert::From<PageViews> for ByteString {
  fn from(page_views: PageViews) -> Self {
    let PageViews { route, count } = page_views;
    let bytes_str = format!("{{route: {route}, count: {count}}}",);
    ByteString::from(bytes_str)
  }
}

// impl std::convert::From<bytestring::ByteString> for PageViews
impl std::convert::From<ByteString> for PageViews {
  fn from(bytes: ByteString) -> Self {
    let bytes_str = bytes.to_string();
    let parts: Vec<&str> = bytes_str.split(",").collect();
    let route = parts[0].split(":").nth(1).unwrap().trim().to_string();
    let count = parts[1]
      .split(":")
      .nth(1)
      .unwrap()
      .trim()
      .parse::<u64>()
      .unwrap();
    PageViews { route, count }
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

impl PartialEq for PageViews {
  fn eq(&self, other: &Self) -> bool {
    self.route == other.route
  }
}

impl Eq for PageViews {}

impl PageViews {
  pub fn with(route: String) -> Self {
    Self {
      route,
      ..Self::default()
    }
  }
}
