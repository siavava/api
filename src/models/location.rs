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

/// A single entry in the visitor location history.
///
/// One document exists per unique city+state pair; `count` accumulates
/// repeat visits and `last_visit_ms` records the most recent one.
///
/// # Fields
///
/// * `city` — City name (e.g. `"San Francisco"`).
/// * `state` — State or region name (e.g. `"California"`).
/// * `count` — Number of recorded visits from this city+state.
/// * `last_visit_ms` — Milliseconds since the Unix epoch of the last visit.
/// * `lat` / `lon` — Geographic coordinates, when the visitor reported them.
#[derive(Debug, Serialize, Clone, Default, PartialEq)]
pub struct LocationHistoryEntry {
  /// City name (e.g. `"San Francisco"`).
  pub city: String,
  /// State or region name (e.g. `"California"`).
  pub state: String,
  /// Number of recorded visits from this city+state.
  pub count: i64,
  /// Milliseconds since the Unix epoch of the last visit.
  pub last_visit_ms: i64,
  /// Latitude in degrees, when reported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lat: Option<f64>,
  /// Longitude in degrees, when reported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lon: Option<f64>,
}

impl LocationHistoryEntry {
  /// Builds a [`LocationHistoryEntry`] from a raw BSON document.
  ///
  /// Tolerates missing fields (empty strings / zero counts) and both
  /// `Int32` and `Int64` representations of `count`.
  ///
  /// # Arguments
  ///
  /// * `document` — A document from the `location_history` collection.
  ///
  /// # Returns
  ///
  /// A `LocationHistoryEntry` with any unreadable field defaulted.
  pub fn from_document(document: &mongodb::bson::Document) -> Self {
    let count = document
      .get_i64("count")
      .or_else(|_| document.get_i32("count").map(i64::from))
      .unwrap_or_default();
    let last_visit_ms = document
      .get_datetime("timestamp")
      .map(|ts| ts.timestamp_millis())
      .unwrap_or_default();
    Self {
      city: document.get_str("city").unwrap_or_default().to_string(),
      state: document.get_str("state").unwrap_or_default().to_string(),
      count,
      last_visit_ms,
      lat: document.get_f64("lat").ok(),
      lon: document.get_f64("lon").ok(),
    }
  }
}

/// A visitor-location event broadcast to connected WebSocket clients.
#[derive(Debug, Clone)]
pub struct LocationEvent {
  /// The updated history entry after recording the visit.
  pub entry: LocationHistoryEntry,
  /// Site namespace the visit was attributed to, when provided.
  pub namespace: Option<String>,
}

/// Location-scoped response sent over the unified WebSocket.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LocationResponse {
  /// A recorded visit, carrying the updated history entry.
  Visit {
    #[serde(flatten)]
    entry: LocationHistoryEntry,
  },
}
