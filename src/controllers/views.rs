use crate::models::views::*;

use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Client;

use mongodb::error::Error as DbError;

// if in production mode, use 'feed' database
// otherwise, use 'feed_dev' database
const DB_NAME: &str = if cfg!(debug_assertions) {
  "feed-dev"
} else {
  "feed"
};

const COLL_NAME: &str = "views";

pub enum ViewsIncrement {
  INCREMENT,
  NOINCREMENT,
}

fn get_views_collection(client: &Client) -> mongodb::Collection<PageViews> {
  client.database(DB_NAME).collection::<PageViews>(COLL_NAME)
}

pub async fn get_views(
  client: &Client,
  route: &str,
  increment: ViewsIncrement,
) -> Result<PageViews, DbError> {
  let collection = get_views_collection(client);

  let filter = doc! { "route": route };
  let res = match increment {
    ViewsIncrement::INCREMENT => {
      let update = doc! { "$inc": { "count": 1 } };
      collection
        .find_one_and_update(filter, update)
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .await
    }
    ViewsIncrement::NOINCREMENT => collection.find_one(filter).await,
  };

  match res {
    Ok(val) => match val {
      Some(val) => Ok(val),
      None => Ok(PageViews {
        route: route.into(),
        ..Default::default()
      }),
    },
    Err(e) => Err(e),
  }
}

// insert one view
pub async fn insert_view(client: &Client, views: PageViews) -> Result<(), DbError> {
  let collection = get_views_collection(client);

  let res = collection.insert_one(views).await;

  match res {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

// insert multiple views
pub async fn insert_views(client: &Client, views: Vec<PageViews>) -> Result<(), DbError> {
  let collection = get_views_collection(client);

  let res = collection.insert_many(views).await;

  match res {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

// delete views
pub async fn delete_views(client: &Client, route: &str) -> Result<(), DbError> {
  let collection = get_views_collection(client);

  let filter = doc! { "route": route };
  let res = collection.delete_one(filter).await;

  match res {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

pub async fn get_all_views(client: &Client) -> Result<Vec<PageViews>, DbError> {
  let collection = get_views_collection(client);

  let res = collection.find(doc! {}).await;

  match res {
    Ok(cursor) => cursor.try_collect().await,
    Err(e) => Err(e),
  }
}

#[macro_export]
macro_rules! views {
  ($client:expr, $target_route:expr, $request_route:expr) => {
    $crate::controllers::views::get_views(
      $client,
      $target_route,
      if $target_route == $request_route {
        $crate::controllers::views::ViewsIncrement::INCREMENT
      } else {
        $crate::controllers::views::ViewsIncrement::NOINCREMENT
      },
    )
    .await
    .unwrap_or_default()
  };
}

#[macro_export]
macro_rules! all_views {
  ($client:expr) => {
    $crate::controllers::views::get_all_views($client)
      .await
      .unwrap_or(vec![])
  };
}
