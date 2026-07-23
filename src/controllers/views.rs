//! # Views Controller
//!
//! Page view counting and retrieval logic.
//!
//! Provides CRUD operations for per-route view counts stored in the `views`
//! MongoDB collection, as well as the [`views!`](crate::views) and
//! [`all_views!`](crate::all_views) convenience macros.

use crate::models::{connect::EventSenders, views::*};

use futures::TryStreamExt;
use mongodb::{Client, bson::doc, error::Error as DbError};

/// Trait abstracting view-count DB operations for testability.
#[allow(async_fn_in_trait)]
pub trait ViewsOps: Send + Sync {
  async fn get_views(
    &self,
    route: &str,
    increment: ViewsIncrement,
  ) -> Result<PageViews, String>;
  async fn get_all_views(
    &self,
    namespace: Option<&str>,
  ) -> Result<Vec<PageViews>, String>;
}

impl ViewsOps for Client {
  async fn get_views(
    &self,
    route: &str,
    increment: ViewsIncrement,
  ) -> Result<PageViews, String> {
    get_views(self, route, increment)
      .await
      .map_err(|e| e.to_string())
  }
  async fn get_all_views(
    &self,
    namespace: Option<&str>,
  ) -> Result<Vec<PageViews>, String> {
    get_all_views(self, namespace)
      .await
      .map_err(|e| e.to_string())
  }
}

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
pub async fn insert_view(
  client: &Client,
  views: PageViews,
) -> Result<(), DbError> {
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
pub async fn insert_views(
  client: &Client,
  views: Vec<PageViews>,
) -> Result<(), DbError> {
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
pub async fn get_all_views(
  client: &Client,
  namespace: Option<&str>,
) -> Result<Vec<PageViews>, DbError> {
  let collection = get_collection(client);
  let options = mongodb::options::FindOptions::builder()
    .sort(doc! { "count": -1 })
    .build();
  let filter = match namespace {
    Some(ns) => doc! {
      "route": mongodb::bson::Regex {
        pattern: format!("^{}:", escape_regex(ns)),
        options: String::new(),
      }
    },
    None => doc! {},
  };
  let views: Vec<PageViews> = collection
    .find(filter)
    .with_options(options)
    .await?
    .try_collect()
    .await?;
  Ok(views)
}

/// Increments the view count for a path and broadcasts the update.
///
/// Called when a client's active path changes. No-ops if `path` is `None`.
/// Also records the view into the hourly activity log.
pub async fn track_page_view(
  client: &Client,
  senders: &EventSenders,
  path: Option<&str>,
) {
  if let Some(path) = path
    && let Ok(updated) =
      get_views(client, path, ViewsIncrement::INCREMENT).await
  {
    if !crate::MONITOR_PATHS.contains(&path) {
      let _ = record_activity(client, path).await;
      if let Some((namespace, label)) = path.split_once(':') {
        let _ = crate::controllers::events::record_event(
          client, namespace, "view", label,
        )
        .await;
      }
    }
    let _ = senders.views.send(ViewEvent { views: updated });
  }
}

/// The hourly view-activity collection name.
const ACTIVITY_COLL_NAME: &str = "view_activity";

/// Records one view into the namespace's hourly activity bucket.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `route` — The namespaced route that was viewed (e.g. `<p>:/about`).
///
/// # Behavior
///
/// Extracts the namespace prefix and increments the bucket for the
/// current UTC hour, creating it on first view. Un-namespaced routes
/// are ignored.
pub async fn record_activity(
  client: &Client,
  route: &str,
) -> Result<(), DbError> {
  let Some((namespace, _)) = route.split_once(':') else {
    return Ok(());
  };
  let hour_ts = chrono::Utc::now().timestamp() / 3600;
  let collection: mongodb::Collection<mongodb::bson::Document> =
    crate::db::collection(client, ACTIVITY_COLL_NAME);
  collection
    .update_one(
      doc! { "namespace": namespace, "hour_ts": hour_ts },
      doc! { "$inc": { "count": 1 } },
    )
    .upsert(true)
    .await?;
  Ok(())
}

/// Reads a namespace's hourly activity for the trailing window.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `namespace` — The site namespace (e.g. `<p>`).
/// * `hours` — Window length in hours, counted back from now.
///
/// # Returns
///
/// [`ActivityBucket`]s in ascending hour order. Hours with no views
/// have no bucket.
pub async fn get_activity(
  client: &Client,
  namespace: &str,
  hours: i64,
) -> Result<Vec<ActivityBucket>, DbError> {
  let cutoff = chrono::Utc::now().timestamp() / 3600 - hours;
  let collection: mongodb::Collection<mongodb::bson::Document> =
    crate::db::collection(client, ACTIVITY_COLL_NAME);
  let mut cursor = collection
    .find(doc! { "namespace": namespace, "hour_ts": { "$gte": cutoff } })
    .sort(doc! { "hour_ts": 1 })
    .await?;
  let mut buckets = Vec::new();
  while let Some(document) = cursor.try_next().await? {
    buckets.push(ActivityBucket {
      hour_ts: document.get_i64("hour_ts").unwrap_or_default(),
      count: document
        .get_i64("count")
        .or_else(|_| document.get_i32("count").map(i64::from))
        .unwrap_or_default(),
    });
  }
  Ok(buckets)
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
    $crate::controllers::views::get_all_views($client, None)
      .await
      .unwrap_or(vec![])
  };
}

/// Escapes regex metacharacters so a namespace prefix matches literally.
///
/// # Arguments
///
/// * `raw` — The untrusted prefix string.
///
/// # Returns
///
/// The input with every regex-significant character backslash-escaped.
fn escape_regex(raw: &str) -> String {
  raw
    .chars()
    .flat_map(|c| match c {
      '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^'
      | '$' | '\\' => vec!['\\', c],
      _ => vec![c],
    })
    .collect()
}
