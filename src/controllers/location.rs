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

use std::pin::Pin;

use crate::models::location::*;
use futures::{FutureExt, future::LocalBoxFuture};
use log::info;
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

/// Returns a handle to the `location_history` collection.
///
/// Each document represents a unique city+state pair with a visit count.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
///
/// # Returns
///
/// A `mongodb::Collection<LocationData>` bound to the `location_history`
/// collection.
fn get_history_collection(client: &Client) -> mongodb::Collection<LocationData> {
  crate::db::collection(client, HISTORY_COLL_NAME)
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
/// The **previous** last-known location (before the update).
pub async fn get_last_and_update<'a>(
  client: &'a Client,
  city: &'a str,
  state: &'a str,
) -> Result<LocationData, DbError> {
  let runners = {
    let fetchers: Vec<Pin<Box<dyn Fn() -> LocalBoxFuture<'a, _>>>> = vec![
      Box::pin(|| update_location_history(client, city, state).boxed_local()),
      Box::pin(|| update_last_location(client, city, state).boxed_local()),
    ];

    fetchers.into_iter().map(|f| f())
  };

  // run all and return the return-value of second future
  let [_, last] = futures::future::join_all(runners).await.try_into().unwrap();
  last
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
/// `Ok(LocationData::default())` — the return value is not meaningful;
/// the caller only cares about the side effect.
async fn update_location_history<'a>(
  client: &'a Client,
  city: &'a str,
  state: &'a str,
) -> Result<LocationData, DbError> {
  let history_collection = get_history_collection(client);

  info!("UPDATING LOCATION HISTORY");

  let _ = history_collection
    .update_one(
      doc! { "city": city, "state": state },
      doc! {
        "$set": { "timestamp": mongodb::bson::DateTime::now() },
        "$inc": { "count": 1 }
      },
    )
    .upsert(true)
    .await;

  // dummy return, we don't care about this value.
  Ok(LocationData::default())
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

  info!("UPDATING LAST LOCATION");

  let found = collection
    .find_one_and_update(doc! {}, doc! { "$set": { "city": city, "state": state } })
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

/// Convenience macro for location operations.
///
/// # Forms
///
/// * `location!(eval $client, eval $city, eval $state)` — updates and
///   returns the **previous** location.
/// * `location!(eval $client)` — reads the last known location without
///   updating.
///
/// Falls back to [`LocationData::default()`] on error.
#[macro_export]
macro_rules! location {
  (eval $client:expr, eval $city:expr, eval $state:expr) => {
    $crate::controllers::location::get_last_and_update($client, $city, $state)
      .await
      .unwrap_or_default()
  };
  (eval $client:expr) => {
    $crate::controllers::location::get_last($client)
      .await
      .unwrap_or_default()
  };
}
