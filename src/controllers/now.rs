//! # Now Controller
//!
//! Two layers for the `now` scope on the unified `/api/connect/`
//! WebSocket:
//!
//! 1. The [`NowOps`] trait — MongoDB CRUD for "now" status entries.
//!    Reads filter out expired entries server-side so a stale row
//!    that hasn't been swept by the TTL index is never returned.
//! 2. [`handle_request`] — dispatches a parsed
//!    [`NowRequest`](crate::models::now::NowRequest) to the matching
//!    op and returns the response paired with a broadcast flag.
//!
//! The `now` scope is one scope among many on the existing connect
//! endpoint — it does **not** open a new endpoint.

use crate::{
  db::collection,
  models::now::{NowEntry, NowKey, NowRequest, NowResponse},
};
use futures::TryStreamExt;
use mongodb::{
  Client,
  bson::{DateTime, doc, to_bson},
};

const COLL_NAME: &str = "now";

/// Trait abstracting `now` DB operations for testability.
#[allow(async_fn_in_trait)]
pub trait NowOps: Send + Sync {
  /// Return all non-expired entries.
  async fn list_now(&self) -> Result<Vec<NowEntry>, String>;
  /// Upsert one entry, returning the persisted value.
  async fn set_now(
    &self,
    key: NowKey,
    value: String,
    url: Option<String>,
    image_url: Option<String>,
    meta: Option<serde_json::Value>,
    ttl_seconds: i64,
  ) -> Result<NowEntry, String>;
  /// Remove one entry by key. Returns whether a document was removed.
  async fn clear_now(&self, key: NowKey) -> Result<bool, String>;
}

impl NowOps for Client {
  async fn list_now(&self) -> Result<Vec<NowEntry>, String> {
    let coll = collection::<NowEntry>(self, COLL_NAME);
    // Keep entries that haven't expired, plus those with no expiry
    // (`expires_at` null or absent — the "forever" entries).
    let filter = doc! { "$or": [
      { "expires_at": null },
      { "expires_at": { "$gt": DateTime::now() } },
    ] };
    let cursor = coll.find(filter).await.map_err(|e| e.to_string())?;
    cursor.try_collect().await.map_err(|e| e.to_string())
  }

  async fn set_now(
    &self,
    key: NowKey,
    value: String,
    url: Option<String>,
    image_url: Option<String>,
    meta: Option<serde_json::Value>,
    ttl_seconds: i64,
  ) -> Result<NowEntry, String> {
    let coll = collection::<NowEntry>(self, COLL_NAME);
    let now = DateTime::now();
    // A non-positive TTL means "never expires" — stored as null so the
    // TTL index leaves it alone.
    let expires_at = if ttl_seconds <= 0 {
      None
    } else {
      let ttl_ms = ttl_seconds.saturating_mul(1000);
      Some(DateTime::from_millis(
        now.timestamp_millis().saturating_add(ttl_ms),
      ))
    };
    let entry = NowEntry {
      key,
      value,
      url,
      image_url,
      meta,
      updated_at: now,
      expires_at,
    };
    let bson = to_bson(&entry).map_err(|e| e.to_string())?;
    let document = bson
      .as_document()
      .cloned()
      .ok_or_else(|| "entry did not serialize to a BSON document".to_string())?;
    coll
      .update_one(doc! { "key": key.as_str() }, doc! { "$set": document })
      .upsert(true)
      .await
      .map_err(|e| e.to_string())?;
    Ok(entry)
  }

  async fn clear_now(&self, key: NowKey) -> Result<bool, String> {
    let coll = collection::<NowEntry>(self, COLL_NAME);
    let result = coll
      .delete_one(doc! { "key": key.as_str() })
      .await
      .map_err(|e| e.to_string())?;
    Ok(result.deleted_count > 0)
  }
}

/// `(response, should_broadcast)` — `true` for mutations whose
/// result every connected client should receive.
pub type Handled = (NowResponse, bool);

/// Dispatches a parsed [`NowRequest`] to its matching operation.
pub async fn handle_request(db: &impl NowOps, request: NowRequest) -> Handled {
  match request {
    NowRequest::Get => match db.list_now().await {
      Ok(entries) => (
        NowResponse::List {
          entries: entries.into_iter().map(Into::into).collect(),
        },
        false,
      ),
      Err(e) => (
        NowResponse::Error {
          message: format!("failed to list now: {e}"),
        },
        false,
      ),
    },
    NowRequest::Set {
      key,
      value,
      url,
      image_url,
      meta,
      ttl_seconds,
    } => match db.set_now(key, value, url, image_url, meta, ttl_seconds).await {
      Ok(entry) => (NowResponse::Updated { entry: entry.into() }, true),
      Err(e) => (
        NowResponse::Error {
          message: format!("failed to set now: {e}"),
        },
        false,
      ),
    },
    NowRequest::Clear { key } => match db.clear_now(key).await {
      Ok(_) => (NowResponse::Cleared { key }, true),
      Err(e) => (
        NowResponse::Error {
          message: format!("failed to clear now: {e}"),
        },
        false,
      ),
    },
  }
}
