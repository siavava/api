use mongodb::Client;
use serde::{Deserialize, Serialize};

pub const DB_NAME: &str = if cfg!(debug_assertions) {
  "feed-dev"
} else {
  "feed"
};

pub fn collection<T>(client: &Client, name: &str) -> mongodb::Collection<T>
where
  T: Serialize + for<'de> Deserialize<'de> + Send + Sync + Unpin,
{
  client.database(DB_NAME).collection::<T>(name)
}
