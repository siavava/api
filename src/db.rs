//! # Database Helpers
//!
//! Shared MongoDB utilities used by all controllers.
//!
//! # Exports
//!
//! * [`DB_NAME`] — The database name (`"feed-dev"` in debug, `"feed"` in
//!   release).
//! * [`collection`] — Returns a typed handle to a named MongoDB collection.

use mongodb::Client;
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
