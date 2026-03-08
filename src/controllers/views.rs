use crate::models::views::*;

use futures::TryStreamExt;
use mongodb::{Client, bson::doc, error::Error as DbError};

const COLL_NAME: &str = "views";

pub fn get_collection(client: &Client) -> mongodb::Collection<PageViews> {
  crate::db::collection(client, COLL_NAME)
}

pub enum ViewsIncrement {
  INCREMENT,
  NOINCREMENT,
}

pub async fn get_views(
  client: &Client,
  route: &str,
  increment: ViewsIncrement,
) -> Result<PageViews, DbError> {
  let collection = get_collection(client);
  let filter = doc! { "route": route };

  let found = match increment {
    ViewsIncrement::INCREMENT => {
      let update = doc! { "$inc": { "count": 1 } };
      collection
        .find_one_and_update(filter, update)
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .await?
    }
    ViewsIncrement::NOINCREMENT => collection.find_one(filter).await?,
  };

  Ok(found.unwrap_or(PageViews {
    route: route.into(),
    ..Default::default()
  }))
}

pub async fn insert_view(client: &Client, views: PageViews) -> Result<(), DbError> {
  let collection = get_collection(client);
  let filter = doc! { "route": &views.route };
  let update = doc! { "$set": { "count": views.count as i32 } };
  let options = mongodb::options::UpdateOptions::builder()
    .upsert(true)
    .build();

  collection
    .update_one(filter, update)
    .with_options(options)
    .await?;
  Ok(())
}

pub async fn insert_views(client: &Client, views: Vec<PageViews>) -> Result<(), DbError> {
  let futures = views.into_iter().map(|view| {
    let client = client.clone();
    async move { insert_view(&client, view).await }
  });

  for result in futures::future::join_all(futures).await {
    result?;
  }
  Ok(())
}

pub async fn delete_views(client: &Client, route: &str) -> Result<(), DbError> {
  let collection = get_collection(client);
  collection.delete_one(doc! { "route": route }).await?;
  Ok(())
}

pub async fn get_all_views(client: &Client) -> Result<Vec<PageViews>, DbError> {
  let collection = get_collection(client);
  let mut views: Vec<PageViews> = collection.find(doc! {}).await?.try_collect().await?;
  views.sort_by(|a, b| b.count.cmp(&a.count));
  Ok(views)
}

#[macro_export]
macro_rules! views {
  ($client:expr, $requested:expr, $location:expr) => {
    $crate::controllers::views::get_views(
      $client,
      $requested,
      if $requested == $location {
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
