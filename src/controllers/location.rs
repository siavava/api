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
const HISTORY_COLL_NAME: &str = "location_history";

pub fn get_collection(client: &Client) -> mongodb::Collection<LocationData> {
  client
    .database(DB_NAME)
    .collection::<LocationData>(COLL_NAME)
}

pub fn get_history_collection(client: &Client) -> mongodb::Collection<LocationData> {
  client
    .database(DB_NAME)
    .collection::<LocationData>(HISTORY_COLL_NAME)
}

pub async fn get_last_and_update(
  client: &Client,
  city: &str,
  state: &str,
) -> Result<LocationData, DbError> {
  let collection = get_collection(client);
  let filter = doc! {};

  update_history(client, city, state);

  let res = {
    let update = doc! {
      "$set": {
        "city": city,
        "state": state,
      }
    };

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

pub fn update_history(client: &Client, city: &str, state: &str) {
  let history_collection = get_history_collection(client);
  
  let filter = doc! {
    "city": city,
    "state": state,
  };

  let update = doc! {
    "$set": {
      "timestamp": mongodb::bson::DateTime::now(),
    },
    "$inc": { "count": 1 }
  };

  actix_web::rt::spawn(async move {
    let _ = history_collection
      .update_one(filter, update)
      .upsert(true)
      .await;
  });
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
