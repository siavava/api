//! # Views Model
//!
//! Data model for page view count tracking.
//!
//! Includes [`ByteString`] conversions for SSE transmission and a custom
//! [`PartialEq`] implementation used by the events broadcaster for filter
//! matching.

use bytestring::ByteString;
use serde::{Deserialize, Serialize};

/// Tracks the view count for a single page route.
///
/// # Fields
///
/// * `route` — The page route being tracked (e.g. `/blog/some-post`).
/// * `count` — Total number of views recorded for this route.
#[derive(Debug, Serialize, Deserialize, Clone, Default, Eq)]
pub struct PageViews {
  /// The page route being tracked (e.g. `/blog/some-post`).
  pub route: String,
  /// Total number of views recorded for this route.
  pub count: u64,
}

/// Equality is based solely on `route`.
///
/// Two `PageViews` with the same route are considered equal regardless of
/// `count`. This is used by
/// [`EventsBroadcaster`](crate::protocol::sse::EventsBroadcaster) to match SSE
/// client filters against incoming changes.
impl PartialEq for PageViews {
  fn eq(&self, other: &Self) -> bool {
    self.route == other.route
  }
}

impl PageViews {
  /// Creates a [`PageViews`] with the given route and a zero count.
  ///
  /// Useful for constructing SSE subscription filters.
  ///
  /// # Arguments
  ///
  /// * `route` — The page route to track.
  ///
  /// # Returns
  ///
  /// A `PageViews` with `count: 0`.
  pub fn with(route: String) -> Self {
    Self {
      route,
      ..Self::default()
    }
  }
}

/// Serializes to JSON for SSE transmission.
impl From<PageViews> for ByteString {
  fn from(val: PageViews) -> Self {
    serde_json::to_string(&val)
      .map(ByteString::from)
      .unwrap_or_default()
  }
}

/// Deserializes from a JSON [`ByteString`].
/// Falls back to [`PageViews::default()`] on parse error.
impl From<ByteString> for PageViews {
  fn from(bytes: ByteString) -> Self {
    serde_json::from_str(bytes.as_ref()).unwrap_or_default()
  }
}

/// One hour of view activity for a site namespace.
///
/// # Fields
///
/// * `hour_ts` — Hours since the Unix epoch (UTC bucket key).
/// * `count` — Views recorded during that hour.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct ActivityBucket {
  /// Hours since the Unix epoch (UTC bucket key).
  pub hour_ts: i64,
  /// Views recorded during that hour.
  pub count: i64,
}

/// A viewer's resolved location, attached to view events when known.
///
/// # Fields
///
/// * `city` / `state` — Place names as reported by the client's
///   geolocation lookup.
/// * `lat` / `lon` — Coordinates, when reported.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ViewerLocation {
  /// City name (e.g. `"San Francisco"`).
  pub city: String,
  /// State or region name (e.g. `"California"`).
  pub state: String,
  /// Latitude in degrees, when reported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lat: Option<f64>,
  /// Longitude in degrees, when reported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lon: Option<f64>,
}

/// Aggregated views from one place for one site namespace.
///
/// One document exists per (namespace, city, state); `count`
/// accumulates views and `last_view_ms` records the most recent one.
///
/// # Fields
///
/// * `city` / `state` — Place names.
/// * `count` — Views recorded from this place.
/// * `last_view_ms` — Milliseconds since the Unix epoch of the last view.
/// * `lat` / `lon` — Coordinates, when any view reported them.
#[derive(Debug, Serialize, Clone, Default, PartialEq)]
pub struct ViewLocationEntry {
  /// City name.
  pub city: String,
  /// State or region name.
  pub state: String,
  /// Views recorded from this place.
  pub count: i64,
  /// Milliseconds since the Unix epoch of the last view.
  pub last_view_ms: i64,
  /// Latitude in degrees, when reported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lat: Option<f64>,
  /// Longitude in degrees, when reported.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lon: Option<f64>,
}

impl ViewLocationEntry {
  /// Builds a [`ViewLocationEntry`] from a raw BSON document,
  /// defaulting any unreadable field.
  pub fn from_document(document: &mongodb::bson::Document) -> Self {
    let count = document
      .get_i64("count")
      .or_else(|_| document.get_i32("count").map(i64::from))
      .unwrap_or_default();
    Self {
      city: document.get_str("city").unwrap_or_default().to_string(),
      state: document.get_str("state").unwrap_or_default().to_string(),
      count,
      last_view_ms: document.get_i64("last_view_ms").unwrap_or_default(),
      lat: document.get_f64("lat").ok(),
      lon: document.get_f64("lon").ok(),
    }
  }
}
