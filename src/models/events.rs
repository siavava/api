//! # Site Events Model
//!
//! Data model for the rolling site-event log: views and visitor
//! arrivals, namespaced per site, consumed by metrics dashboards.

use serde::Serialize;

/// One logged site event.
///
/// # Fields
///
/// * `kind` — Event kind: `"view"` or `"visit"`.
/// * `label` — Human-readable subject: a de-namespaced path for views,
///   a `City, State` pair for visits.
/// * `ts_ms` — Milliseconds since the Unix epoch.
#[derive(Debug, Serialize, Clone, Default, PartialEq, Eq)]
pub struct SiteEvent {
  /// Event kind: `"view"` or `"visit"`.
  pub kind: String,
  /// Human-readable subject of the event.
  pub label: String,
  /// Milliseconds since the Unix epoch.
  pub ts_ms: i64,
}

impl SiteEvent {
  /// Builds a [`SiteEvent`] from a raw BSON document, defaulting any
  /// unreadable field.
  ///
  /// # Arguments
  ///
  /// * `document` — A document from the `site_events` collection.
  ///
  /// # Returns
  ///
  /// A `SiteEvent` with missing fields defaulted.
  pub fn from_document(document: &mongodb::bson::Document) -> Self {
    Self {
      kind: document.get_str("kind").unwrap_or_default().to_string(),
      label: document.get_str("label").unwrap_or_default().to_string(),
      ts_ms: document.get_i64("ts_ms").unwrap_or_default(),
    }
  }
}
