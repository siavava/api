//! Data models and WebSocket protocol types for the `now` scope.
//!
//! "Now" entries are short-lived owner-published status slots
//! (currently watching, reading, working on, etc.) surfaced in the
//! blog's dynamic island. Each slot has a TTL — expired entries are
//! filtered out by [`list_now`](crate::controllers::now::NowOps::list_now)
//! and auto-removed by MongoDB's TTL index on `expires_at`.

use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// A slot identifier.
///
/// Each variant maps 1:1 to a `now` document keyed by its string
/// form (snake_case). Music playback lives in the separate
/// [`playback`](crate::models::playback) scope and is not represented
/// here.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum NowKey {
  Watching,
  Reading,
  WorkingOn,
  Status,
}

impl NowKey {
  /// String form used as the `key` field in MongoDB documents.
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Watching => "watching",
      Self::Reading => "reading",
      Self::WorkingOn => "working_on",
      Self::Status => "status",
    }
  }
}

/// One "now" status entry as stored in MongoDB.
///
/// Timestamps are native `bson::DateTime` so the TTL index on
/// `expires_at` works and date comparisons in queries are correct.
/// For the wire (JSON over WebSocket) this is converted to
/// [`NowEntryView`], whose timestamps are RFC 3339 strings — bson 2.x
/// serializes `DateTime` as Extended JSON (`{"$date": …}`) for
/// serde_json, which the browser's `Date.parse` can't read.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowEntry {
  /// Slot identifier (unique across the collection).
  pub key: NowKey,
  /// Display value (e.g. `"Severance S2E3"`).
  pub value: String,
  /// Optional companion URL (e.g. trailer, book page).
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub url: Option<String>,
  /// Optional thumbnail used as the slot's "album art" — rendered in
  /// the dynamic island's pill, secondary pip, and expanded card.
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub image_url: Option<String>,
  /// Optional structured detail (e.g. `{ "author": "Hesse" }`).
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub meta: Option<serde_json::Value>,
  /// Server-side write time.
  pub updated_at: DateTime,
  /// Server-side expiry — entry is dropped at this instant by the TTL
  /// index. `None` means the entry never expires: it's stored as BSON
  /// null, which the TTL index ignores (it only sweeps date values).
  pub expires_at: Option<DateTime>,
}

/// Wire representation of [`NowEntry`] sent to clients over the
/// WebSocket. Identical to `NowEntry` except timestamps are RFC 3339
/// strings (e.g. `"2026-05-21T12:34:56.789+00:00"`) so the frontend
/// can parse them with `Date.parse`.
#[derive(Debug, Clone, Serialize)]
pub struct NowEntryView {
  pub key: NowKey,
  pub value: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<serde_json::Value>,
  pub updated_at: String,
  /// RFC 3339 expiry, or `None` for an entry that never expires.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expires_at: Option<String>,
}

/// Formats a `bson::DateTime` as an RFC 3339 string, falling back to
/// an empty string if the conversion fails (it shouldn't for valid
/// dates).
fn rfc3339(dt: DateTime) -> String {
  dt.try_to_rfc3339_string().unwrap_or_default()
}

impl From<NowEntry> for NowEntryView {
  fn from(e: NowEntry) -> Self {
    Self {
      key: e.key,
      value: e.value,
      url: e.url,
      image_url: e.image_url,
      meta: e.meta,
      updated_at: rfc3339(e.updated_at),
      expires_at: e.expires_at.map(rfc3339),
    }
  }
}

/// Incoming WebSocket message for the `now` scope.
///
/// Discriminated by the `"action"` JSON field.
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum NowRequest {
  /// Fetch all non-expired entries.
  Get,
  /// Upsert one entry. Broadcasts on success.
  Set {
    key: NowKey,
    value: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    image_url: Option<String>,
    #[serde(default)]
    meta: Option<serde_json::Value>,
    /// Seconds until expiry. A value `<= 0` means the entry never
    /// expires (`expires_at` is stored as null).
    ttl_seconds: i64,
  },
  /// Remove one entry by key. Broadcasts on success.
  Clear { key: NowKey },
}

/// Outgoing WebSocket message for the `now` scope.
///
/// Discriminated by the `"type"` JSON field.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum NowResponse {
  /// Snapshot of all non-expired entries.
  List { entries: Vec<NowEntryView> },
  /// One entry was upserted.
  Updated { entry: NowEntryView },
  /// One entry was cleared.
  Cleared { key: NowKey },
  /// A human-readable error.
  Error { message: String },
}

/// A "now"-scope event broadcast to all connected clients.
#[derive(Debug, Clone)]
pub struct NowEvent {
  pub response: NowResponse,
}
