//! # Comments Controller
//!
//! Core logic for blog comment CRUD operations, including likes and
//! recursive reply-tree management.
//!
//! All functions operate against the `comments` MongoDB collection and return
//! domain types from [`crate::models::comments`].

use crate::models::comments::{BlogComment, CommentEdit, PopulatedComment};

use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{Client, bson::doc, bson::oid::ObjectId, error::Error as DbError};

const COLL_NAME: &str = "comments";

/// Returns a handle to the `comments` MongoDB collection.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
///
/// # Returns
///
/// A `mongodb::Collection<BlogComment>` bound to the `comments` collection.
fn get_collection(client: &Client) -> mongodb::Collection<BlogComment> {
  crate::db::collection(client, COLL_NAME)
}

/// Parses a list of hex `ObjectId` strings, skipping any that are invalid.
///
/// # Arguments
///
/// * `ids` — Slice of hex-encoded ObjectId strings.
///
/// # Returns
///
/// A `Vec<ObjectId>` containing only the successfully parsed entries.
fn parse_oids(ids: &[String]) -> Vec<ObjectId> {
  ids
    .iter()
    .filter_map(|s| ObjectId::parse_str(s).ok())
    .collect()
}

/// Applies an update to a comment, then re-fetches it with its reply tree
/// populated.
///
/// # Arguments
///
/// * `collection` — Handle to the `comments` collection.
/// * `id` — The `ObjectId` of the comment to update.
/// * `update` — A BSON update document (e.g. `$set`, `$inc`).
///
/// # Returns
///
/// * `Ok(Some(populated))` — The updated comment with nested replies.
/// * `Ok(None)` — The comment no longer exists after the update
///   (should not happen in practice).
async fn update_and_populate(
  collection: &mongodb::Collection<BlogComment>,
  id: &ObjectId,
  update: mongodb::bson::Document,
) -> Result<Option<PopulatedComment>, DbError> {
  let filter = doc! { "_id": id };
  collection.update_one(filter.clone(), update).await?;
  match collection.find_one(filter).await? {
    Some(comment) => Ok(Some(populate_replies(collection.clone(), comment).await?)),
    None => Ok(None),
  }
}

/// Creates a new comment and inserts it into the database.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `comment` — The comment payload.
///   Fields `created_time`, `edited_time`, `likes`, and `replies` are
///   overwritten with server-side defaults regardless of client input.
/// * `reply_to` — If `Some`, this comment is a reply — the new comment's
///   ID is pushed into the parent's `replies` array.
///
/// # Returns
///
/// The inserted [`BlogComment`] with its generated `id`.
pub async fn create_comment(
  client: &Client,
  comment: BlogComment,
  reply_to: Option<&ObjectId>,
) -> Result<BlogComment, DbError> {
  let collection = get_collection(client);
  let now = Utc::now().to_rfc3339();
  let comment = BlogComment {
    created_time: now,
    edited_time: None,
    likes: 0,
    reply_to: reply_to.map(|oid| oid.to_hex()),
    replies: vec![],
    ..comment
  };
  let result = collection.insert_one(&comment).await?;
  let id = result.inserted_id.as_object_id();

  // If this is a reply, push our new id into the parent's replies array
  if let (Some(parent_id), Some(new_id)) = (reply_to, &id) {
    let filter = doc! { "_id": parent_id };
    let update = doc! { "$push": { "replies": new_id.to_hex() } };
    collection.update_one(filter, update).await?;
  }

  Ok(BlogComment { id, ..comment })
}

/// Updates a comment's text and sets `edited_time` to now.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `id` — The `ObjectId` of the comment to edit.
/// * `edit` — Contains the new `text` value.
///
/// # Returns
///
/// The updated comment with populated replies, or `None` if no comment
/// with that `id` exists.
pub async fn edit_comment(
  client: &Client,
  id: &ObjectId,
  edit: CommentEdit,
) -> Result<Option<PopulatedComment>, DbError> {
  let collection = get_collection(client);
  let now = Utc::now().to_rfc3339();
  let update = doc! {
    "$set": {
      "text": &edit.text,
      "edited_time": &now,
    }
  };
  update_and_populate(&collection, id, update).await
}

/// Increments a comment's `likes` count by 1.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `id` — The `ObjectId` of the comment to like.
///
/// # Returns
///
/// The updated comment with populated replies, or `None` if no comment
/// with that `id` exists.
///
/// > **Note:** Likes are *not* idempotent — each call adds one like.
pub async fn like_comment(
  client: &Client,
  id: &ObjectId,
) -> Result<Option<PopulatedComment>, DbError> {
  let collection = get_collection(client);
  let update = doc! { "$inc": { "likes": 1 } };
  update_and_populate(&collection, id, update).await
}

/// Deletes a comment and all of its replies recursively.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `id` — The `ObjectId` of the comment to delete.
///
/// # Behavior
///
/// 1. Looks up the comment by `id`.
/// 2. Recursively deletes all nested replies concurrently via `join_all`.
/// 3. If the comment is itself a reply, pulls its ID from the parent's
///    `replies` array.
/// 4. Deletes the comment document itself.
///
/// # Returns
///
/// The total number of documents deleted (the comment itself plus all
/// nested replies).
/// Returns `0` if no comment with that `id` exists.
pub async fn delete_comment(client: &Client, id: &ObjectId) -> Result<(u64, Option<String>), DbError> {
  let collection = get_collection(client);

  // Find the comment first so we can access its replies and parent
  let filter = doc! { "_id": id };
  let comment = match collection.find_one(filter.clone()).await? {
    Some(c) => c,
    None => return Ok((0, None)),
  };

  let path = comment.path.clone();

  // Recursively delete all replies concurrently
  let reply_futures: Vec<_> = parse_oids(&comment.replies)
    .into_iter()
    .map(|reply_oid| Box::pin(async move { delete_comment(client, &reply_oid).await }))
    .collect();

  let mut deleted_count: u64 = 0;
  for result in futures::future::join_all(reply_futures).await {
    deleted_count += result?.0;
  }

  // Remove this comment's ID from the parent's replies array
  if let Some(ref parent_id_str) = comment.reply_to
    && let Ok(parent_oid) = ObjectId::parse_str(parent_id_str)
  {
    let parent_filter = doc! { "_id": &parent_oid };
    let update = doc! { "$pull": { "replies": id.to_hex() } };
    collection.update_one(parent_filter, update).await?;
  }

  // Delete the comment itself
  let result = collection.delete_one(filter).await?;
  Ok((deleted_count + result.deleted_count, Some(path)))
}

/// Lists all top-level comments for a given page path, each with its full
/// reply tree populated.
///
/// # Arguments
///
/// * `client` — The MongoDB client connection.
/// * `path` — The page path to filter by (e.g. `/blog/some-post`).
///
/// # Behavior
///
/// Only comments where `reply_to` is `null` are returned at the top level;
/// nested replies are resolved recursively via [`populate_replies`].
///
/// # Returns
///
/// A `Vec<PopulatedComment>` of top-level comments with nested reply trees.
pub async fn list_comments(client: &Client, path: &str) -> Result<Vec<PopulatedComment>, DbError> {
  let collection = get_collection(client);

  // Only fetch top-level comments
  let filter = doc! { "path": path, "reply_to": null };
  let top_level: Vec<BlogComment> = collection.find(filter).await?.try_collect().await?;

  let mut populated = Vec::with_capacity(top_level.len());
  for comment in top_level {
    let p = populate_replies(collection.clone(), comment).await?;
    populated.push(p);
  }

  Ok(populated)
}

/// Recursively fetches and nests a comment's reply tree.
///
/// Converts a [`BlogComment`] (which stores replies as a flat list of ID
/// strings) into a [`PopulatedComment`] with fully resolved, nested `replies`.
///
/// # Arguments
///
/// * `collection` — Handle to the `comments` collection.
/// * `comment` — The comment whose replies should be populated.
///
/// # Returns
///
/// A [`PopulatedComment`] with its full reply tree resolved.
/// Uses `Box::pin` for async recursion.
async fn populate_replies(
  collection: mongodb::Collection<BlogComment>,
  comment: BlogComment,
) -> Result<PopulatedComment, DbError> {
  if comment.replies.is_empty() {
    return Ok(PopulatedComment::from_comment(comment, vec![]));
  }

  let reply_oids = parse_oids(&comment.replies);
  let filter = doc! { "_id": { "$in": &reply_oids } };
  let reply_comments: Vec<BlogComment> = collection.find(filter).await?.try_collect().await?;

  let futures: Vec<_> = reply_comments
    .into_iter()
    .map(|reply| Box::pin(populate_replies(collection.clone(), reply)))
    .collect();

  let mut populated_replies = Vec::with_capacity(futures.len());
  for result in futures::future::join_all(futures).await {
    populated_replies.push(result?);
  }

  Ok(PopulatedComment::from_comment(comment, populated_replies))
}
