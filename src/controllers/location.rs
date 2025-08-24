use crate::models::location::*;
use mongodb::{Client, bson::doc, error::Error as DbError};
use tracing::info;

// if in production mode, use 'feed' database
// otherwise, use 'feed-dev' database
const DB_NAME: &str = if cfg!(debug_assertions) {
  "feed-dev"
} else {
  "feed"
};

const COLL_NAME: &str = "location";

pub fn get_collection(client: &Client) -> mongodb::Collection<LocationData> {
  client
    .database(DB_NAME)
    .collection::<LocationData>(COLL_NAME)
}

pub async fn get_last_and_update(
  client: &Client,
  city: &str,
  state: &str,
) -> Result<LocationData, DbError> {
  let collection = get_collection(client);
  let filter = doc! {};
  let res = {
    let update = doc! {
      "$set": {
        "city": city,
        "state": state,
      }
    };

    info!("UPDATE: {:?}", update);

    collection
      .find_one_and_update(filter, update)
      .upsert(true)
      .return_document(mongodb::options::ReturnDocument::Before)
      .await
  };

  info!("res: {:?}", res);

  match res {
    Ok(val) => match val {
      Some(val) => Ok(val),
      None => Ok(LocationData::default()),
    },
    Err(e) => Err(e),
  }
}

pub async fn get_last(client: &Client) -> Result<LocationData, DbError> {
  let collection = get_collection(client);
  let filter = doc! {};
  let res = collection.find_one(filter).await;

  match res {
    Ok(val) => match val {
      Some(val) => Ok(val),
      None => Ok(LocationData::default()),
    },
    Err(e) => Err(e),
  }
}

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
