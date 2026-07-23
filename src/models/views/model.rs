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
