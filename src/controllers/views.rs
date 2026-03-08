//! # Views Controller
//!
//! Page view counting and retrieval logic.
//!
//! Provides CRUD operations for per-route view counts stored in the `views`
//! MongoDB collection, as well as the [`views!`](crate::views) and
//! [`all_views!`](crate::all_views) convenience macros.

use crate::models::views::*;

use futures::TryStreamExt;
use mongodb::{Client, bson::doc, error::Error as DbError};

const COLL_NAME: &str = "views";

/// Returns a handle to the `views` MongoDB collection.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
///
/// # Returns
///
/// A `mongodb::Collection<PageViews>` bound to the `views` collection.
pub fn get_collection(client: &Client) -> mongodb::Collection<PageViews> {
  crate::db::collection(client, COLL_NAME)
}

/// Controls whether [`get_views`] atomically increments the count.
pub enum ViewsIncrement {
  /// Increment the view count by 1 before returning.
  INCREMENT,
  /// Read the current count without modifying it.
  NOINCREMENT,
}

/// Fetches the view count for a route, optionally incrementing it first.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `route` — The page route to look up (e.g. `/blog/some-post`).
/// * `increment` — Whether to atomically bump the count.
///   Uses `find_one_and_update` with `upsert: true` when incrementing.
///
/// # Returns
///
/// The [`PageViews`] for the route.
/// If the route has no existing document and `NOINCREMENT` is used, returns
/// a default `PageViews` with count `0`.
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

/// Upserts a single [`PageViews`] record, setting `count` to the given value.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `views` — The route + count to store.
///   Creates the document if it doesn't exist.
///
/// # Returns
///
/// `Ok(())` on success, or a `DbError` on failure.
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

/// Upserts multiple [`PageViews`] records concurrently via `join_all`.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `views` — The records to upsert.
///
/// # Returns
///
/// `Ok(())` on success. Fails on the first error encountered.
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

/// Deletes the view-count document for the given route.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `route` — The route whose view-count document should be removed.
///
/// # Returns
///
/// `Ok(())` on success, or a `DbError` on failure.
pub async fn delete_views(client: &Client, route: &str) -> Result<(), DbError> {
  let collection = get_collection(client);
  collection.delete_one(doc! { "route": route }).await?;
  Ok(())
}

/// Returns all view-count documents, sorted by count descending.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
///
/// # Returns
///
/// `Ok(Vec<PageViews>)` ordered from most-viewed to least-viewed.
pub async fn get_all_views(client: &Client) -> Result<Vec<PageViews>, DbError> {
  let collection = get_collection(client);
  let mut views: Vec<PageViews> = collection.find(doc! {}).await?.try_collect().await?;
  views.sort_by(|a, b| b.count.cmp(&a.count));
  Ok(views)
}

/// Convenience macro: fetches views for `$requested`, incrementing only if
/// `$requested == $location` (i.e. the viewer is on that page).
///
/// # Usage
///
/// ```ignore
/// let page_views = views![&db_client, &requested_str, &location_str];
/// ```
///
/// Falls back to [`PageViews::default()`] on error.
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

/// Convenience macro: fetches all view-count documents.
///
/// # Usage
///
/// ```ignore
/// let all = all_views![&db_client];
/// ```
///
/// Falls back to an empty `Vec` on error.
#[macro_export]
macro_rules! all_views {
  ($client:expr) => {
    $crate::controllers::views::get_all_views($client)
      .await
      .unwrap_or(vec![])
  };
}
