/// Views model
///
/// This model is used to store the views of the routes
///
use bytestring::ByteString;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

/// Views struct
///
/// This struct is used to store the views of the routes
///
/// It tracks the route and the count of the views
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LocationData {
  pub city: String,
  pub state: String,
}

// impl debug for Views
impl std::fmt::Debug for LocationData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Location {{ city: {}, state: {} }}",
      self.city, self.state
    )
  }
}

// convert PagViews to bytestring
impl std::convert::From<LocationData> for ByteString {
  fn from(location: LocationData) -> Self {
    // let LocationData { route, count } = page_views;
    // let bytes_str = format!("{{route:\"{route}\",count:\"{count}\"}}",);
    let bytes_str = serde_json::to_string(&location);
    match bytes_str {
      Ok(value) => ByteString::from(value),
      Err(_) => ByteString::default(),
    }
  }
}

// impl std::convert::From<bytestring::ByteString> for LocationData
impl std::convert::From<ByteString> for LocationData {
  fn from(bytes: ByteString) -> Self {
    let bytes_str = bytes.to_string();
    let parts: Vec<&str> = bytes_str.split(",").collect();
    let city = parts[0].split(":").nth(1).unwrap().trim().to_string();
    let state = parts[1].split(":").nth(1).unwrap().trim().to_string();
    LocationData { city, state }
  }
}

// implement Default for Views

impl PartialEq for LocationData {
  fn eq(&self, other: &Self) -> bool {
    self.city == other.city && self.state == other.state

    // ? DEPRECATED VERSION
    // // let conditions = [
    // //   // self.route.is_empty() && other.route.is_empty(), // if other has "empty" route, pass
    // //   self.route == other.route, // otherwise pass if routes equivalent
    // // ];
    // //
    // // conditions.iter().any(|x| *x)
  }
}

impl Eq for LocationData {}

impl LocationData {
  pub fn with(city: String, state: String) -> Self {
    Self { city, state }
  }
}
