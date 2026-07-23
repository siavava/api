//! # Location Controller
//!
//! Manages user location tracking — both the single "last known" location and
//! a per-city/state visit history log.
//!
//! Uses two MongoDB collections:
//! - **`location`** — stores one document: the most recent location.
//! - **`location_history`** — one document per unique city+state pair, with
//!   a visit count and timestamp.
//!
//! Also exports the [`location!`](crate::location) convenience macro.

use crate::models::location::*;
use mongodb::{Client, bson::doc, error::Error as DbError};

const COLL_NAME: &str = "location";
const HISTORY_COLL_NAME: &str = "location_history";

/// Returns a handle to the `location` MongoDB collection.
///
/// This collection stores a single document representing the most-recent
/// reported location.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
///
/// # Returns
///
/// A `mongodb::Collection<LocationData>` bound to the `location` collection.
pub fn get_collection(client: &Client) -> mongodb::Collection<LocationData> {
  crate::db::collection(client, COLL_NAME)
}

/// Records a new location: updates the history log and overwrites the
/// "last known" location, all concurrently.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `city` — City name to record.
/// * `state` — State or region name to record.
///
/// # Behavior
///
/// Runs two operations concurrently:
/// 1. Upserts the history entry for the city+state pair
///    (incrementing its visit count).
/// 2. Overwrites the "last known" location document.
///
/// # Returns
///
/// The **previous** last-known location (before the update), together
/// with the updated history entry for the recorded city+state.
pub async fn get_last_and_update(
  client: &Client,
  city: &str,
  state: &str,
  namespace: Option<&str>,
  coords: Option<(f64, f64)>,
) -> Result<(LocationData, LocationHistoryEntry), DbError> {
  let (entry, last) = tokio::try_join!(
    update_location_history(client, city, state, namespace, coords),
    update_last_location(client, city, state),
  )?;
  Ok((last, entry))
}

/// Upserts the location history entry for a city+state pair.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `city` — City name to record.
/// * `state` — State or region name to record.
///
/// # Behavior
///
/// - Increments the `count` field by 1.
/// - Sets `timestamp` to the current time.
/// - Creates the document if it doesn't already exist (`upsert: true`).
///
/// # Returns
///
/// The updated [`LocationHistoryEntry`] after the upsert.
async fn update_location_history(
  client: &Client,
  city: &str,
  state: &str,
  namespace: Option<&str>,
  coords: Option<(f64, f64)>,
) -> Result<LocationHistoryEntry, DbError> {
  let history_collection: mongodb::Collection<mongodb::bson::Document> =
    crate::db::collection(client, HISTORY_COLL_NAME);

  let mut set = doc! { "timestamp": mongodb::bson::DateTime::now() };
  if let Some(ns) = namespace {
    set.insert("namespace", ns);
  }
  if let Some((lat, lon)) = coords {
    set.insert("lat", lat);
    set.insert("lon", lon);
  }

  let updated = history_collection
    .find_one_and_update(
      doc! { "city": city, "state": state, "namespace": namespace },
      doc! { "$set": set, "$inc": { "count": 1 } },
    )
    .upsert(true)
    .return_document(mongodb::options::ReturnDocument::After)
    .await?;

  Ok(
    updated
      .as_ref()
      .map(LocationHistoryEntry::from_document)
      .unwrap_or_default(),
  )
}

/// Overwrites the single "last known location" document with a new
/// city+state.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `city` — City name to set.
/// * `state` — State or region name to set.
///
/// # Returns
///
/// `Ok(LocationData)` — the **previous** location (before the update).
/// Falls back to [`LocationData::default()`] if no document existed.
async fn update_last_location<'a>(
  client: &'a Client,
  city: &'a str,
  state: &'a str,
) -> Result<LocationData, DbError> {
  let collection = get_collection(client);

  let found = collection
    .find_one_and_update(
      doc! {},
      doc! { "$set": { "city": city, "state": state } },
    )
    .upsert(true)
    .return_document(mongodb::options::ReturnDocument::Before)
    .await?;

  Ok(found.unwrap_or_default())
}

/// Returns the last known location without updating it.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
///
/// # Returns
///
/// `Ok(LocationData)` — the current location, or
/// [`LocationData::default()`] if no document exists.
pub async fn get_last(client: &Client) -> Result<LocationData, DbError> {
  let collection = get_collection(client);
  Ok(collection.find_one(doc! {}).await?.unwrap_or_default())
}

/// Reads the full visitor location history, sorted by visit count
/// descending.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
///
/// # Returns
///
/// All [`LocationHistoryEntry`] documents from `location_history`,
/// most-visited first. Unreadable fields default rather than erroring.
pub async fn get_location_history(
  client: &Client,
  namespace: Option<&str>,
) -> Result<Vec<LocationHistoryEntry>, DbError> {
  use futures::TryStreamExt;

  let collection: mongodb::Collection<mongodb::bson::Document> =
    crate::db::collection(client, HISTORY_COLL_NAME);

  let filter = match namespace {
    Some(ns) => doc! { "namespace": ns },
    None => doc! {},
  };

  let mut cursor = collection.find(filter).sort(doc! { "count": -1 }).await?;
  let mut entries = Vec::new();
  while let Some(document) = cursor.try_next().await? {
    entries.push(LocationHistoryEntry::from_document(&document));
  }
  Ok(entries)
}

/// Convenience macro for location operations.
///
/// # Forms
///
/// * `location!(eval $client)` — reads the last known location without
///   updating.
///
/// Falls back to [`LocationData::default()`] on error.
#[macro_export]
macro_rules! location {
  (eval $client:expr) => {
    $crate::controllers::location::get_last($client)
      .await
      .unwrap_or_default()
  };
}
