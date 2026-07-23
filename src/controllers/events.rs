//! # Events Controller
//!
//! Rolling site-event log: records views and visitor arrivals per
//! site namespace, and serves the recent window to dashboards. The
//! log self-prunes to the trailing thirty days on write.

use crate::models::events::SiteEvent;
use futures::TryStreamExt;
use mongodb::{Client, bson::doc, error::Error as DbError};

const COLL_NAME: &str = "site_events";

/// Thirty days, in milliseconds — the retention window.
const RETENTION_MS: i64 = 30 * 24 * 3600 * 1000;

/// Returns a handle to the `site_events` collection.
fn get_collection(
  client: &Client,
) -> mongodb::Collection<mongodb::bson::Document> {
  crate::db::collection(client, COLL_NAME)
}

/// Records one site event and prunes entries past retention.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `namespace` — The site namespace (e.g. `<p>`).
/// * `kind` — Event kind: `"view"` or `"visit"`.
/// * `label` — Human-readable subject of the event.
pub async fn record_event(
  client: &Client,
  namespace: &str,
  kind: &str,
  label: &str,
) -> Result<(), DbError> {
  let collection = get_collection(client);
  let now_ms = chrono::Utc::now().timestamp_millis();

  collection
    .insert_one(doc! {
      "namespace": namespace,
      "kind": kind,
      "label": label,
      "ts_ms": now_ms,
    })
    .await?;
  collection
    .delete_many(doc! { "ts_ms": { "$lt": now_ms - RETENTION_MS } })
    .await?;
  Ok(())
}

/// Reads a namespace's most recent events, newest first.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `namespace` — The site namespace (e.g. `<p>`).
/// * `limit` — Maximum number of events to return.
///
/// # Returns
///
/// Up to `limit` [`SiteEvent`]s in descending time order.
pub async fn get_events(
  client: &Client,
  namespace: &str,
  limit: i64,
) -> Result<Vec<SiteEvent>, DbError> {
  let collection = get_collection(client);
  let mut cursor = collection
    .find(doc! { "namespace": namespace })
    .sort(doc! { "ts_ms": -1 })
    .limit(limit)
    .await?;
  let mut events = Vec::new();
  while let Some(document) = cursor.try_next().await? {
    events.push(SiteEvent::from_document(&document));
  }
  Ok(events)
}
