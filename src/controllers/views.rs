use crate::models::views::*;

use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Client;

use mongodb::error::Error as DbError;

const DB_NAME: &str = "feed";
const COLL_NAME: &str = "views";

pub enum ViewsIncrement {
  INCREMENT,
  NOINCREMENT,
}

pub async fn get_views(
  client: &Client,
  route: &str,
  increment: ViewsIncrement,
) -> Result<PageViews, DbError> {
  let collection = client.database(DB_NAME).collection::<PageViews>(COLL_NAME);

  let filter = doc! { "route": route };
  // let update = doc! { "$inc": { "count": 1 } };
  let update = doc! { "$inc": { "count": match increment {
    ViewsIncrement::INCREMENT => 1,
    ViewsIncrement::NOINCREMENT => 0,
  } } };

  let res = collection
    .find_one_and_update(filter, update)
    .upsert(true)
    .return_document(mongodb::options::ReturnDocument::After)
    .await;

  println!("res: {:?}", res);

  match res {
    Ok(val) => Ok(val.unwrap()),
    Err(e) => Err(e),
  }
}

pub async fn get_all_views(client: &Client) -> Result<Vec<PageViews>, DbError> {
  let collection = client.database(DB_NAME).collection::<PageViews>(COLL_NAME);

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
    .unwrap()
  };
}

#[macro_export]
macro_rules! all_views {
  ($client:expr) => {
    $crate::controllers::views::get_all_views($client)
      .await
      .unwrap()
  };
}
