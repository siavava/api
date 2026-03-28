//! # Database Helpers
//!
//! Shared MongoDB utilities used by all controllers.
//!
//! # Exports
//!
//! * [`DB_NAME`] — The database name (`"feed-dev"` in debug, `"feed"` in
//!   release).
//! * [`collection`] — Returns a typed handle to a named MongoDB collection.

use mongodb::{Client, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};

/// Database name, chosen at compile time.
///
/// * **Debug builds** — `"feed-dev"`
/// * **Release builds** — `"feed"`
pub const DB_NAME: &str = if cfg!(debug_assertions) {
  "feed-dev"
} else {
  "feed"
};

/// Returns a typed handle to a named MongoDB collection.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `name` — The collection name within the [`DB_NAME`] database.
///
/// # Returns
///
/// A `mongodb::Collection<T>` ready for queries.
pub fn collection<T>(client: &Client, name: &str) -> mongodb::Collection<T>
where
  T: Serialize + for<'de> Deserialize<'de> + Send + Sync + Unpin,
{
  client.database(DB_NAME).collection::<T>(name)
}

/// Parses a hex string as a MongoDB `ObjectId`.
///
/// Returns a human-readable error message on failure, suitable for
/// sending back to API clients.
pub fn parse_oid(id: &str) -> Result<ObjectId, String> {
  ObjectId::parse_str(id).map_err(|e| format!("invalid id: {e}"))
}

/// Parses a list of hex `ObjectId` strings, skipping any that are invalid.
pub fn parse_oids(ids: &[String]) -> Vec<ObjectId> {
  ids
    .iter()
    .filter_map(|s| ObjectId::parse_str(s).ok())
    .collect()
}

#[cfg(test)]
mod tests;
