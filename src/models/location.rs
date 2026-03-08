//! # Location Model
//!
//! Data model for location tracking (city + state).
//!
//! Includes [`ByteString`] conversions for SSE transmission.

use bytestring::ByteString;
use serde::{Deserialize, Serialize};

/// The most recently reported location (city + state).
///
/// # Fields
///
/// * `city` — City name (e.g. `"San Francisco"`).
/// * `state` — State or region name (e.g. `"California"`).
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct LocationData {
  /// City name (e.g. `"San Francisco"`).
  pub city: String,
  /// State or region name (e.g. `"California"`).
  pub state: String,
}

impl LocationData {
  /// Constructs a [`LocationData`] with the given city and state.
  ///
  /// # Arguments
  ///
  /// * `city` — City name.
  /// * `state` — State or region name.
  ///
  /// # Returns
  ///
  /// A new `LocationData` instance.
  pub fn with(city: String, state: String) -> Self {
    Self { city, state }
  }
}

/// Serializes to JSON for SSE transmission.
impl From<LocationData> for ByteString {
  fn from(val: LocationData) -> Self {
    serde_json::to_string(&val)
      .map(ByteString::from)
      .unwrap_or_default()
  }
}

/// Deserializes from a JSON [`ByteString`].
/// Falls back to [`LocationData::default()`] on parse error.
impl From<ByteString> for LocationData {
  fn from(bytes: ByteString) -> Self {
    serde_json::from_str(bytes.as_ref()).unwrap_or_default()
  }
}
