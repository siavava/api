//! # Comments Controller
//!
//! Core logic for blog comment CRUD operations, including likes and
//! recursive reply-tree management.
//!
//! All functions operate against the `comments` MongoDB collection and return
//! domain types from [`crate::models::comments`].

use crate::{
  db::parse_oids,
  models::comments::{BlogComment, CommentEdit, PopulatedComment},
};

use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::{
  Client,
  bson::{doc, oid::ObjectId},
  error::Error as DbError,
};
use std::collections::{HashMap, HashSet};

/// Trait abstracting comment DB operations for testability.
#[allow(async_fn_in_trait)]
pub trait CommentOps: Send + Sync {
  async fn create_comment(
    &self,
    comment: BlogComment,
    reply_to: Option<&ObjectId>,
  ) -> Result<BlogComment, String>;
  async fn edit_comment(
    &self,
    id: &ObjectId,
    edit: CommentEdit,
  ) -> Result<Option<PopulatedComment>, String>;
  async fn like_comment(
    &self,
    id: &ObjectId,
  ) -> Result<Option<PopulatedComment>, String>;
  async fn delete_comment(
    &self,
    id: &ObjectId,
  ) -> Result<(u64, Option<String>), String>;
  async fn list_comments(
    &self,
    path: &str,
    actor: Option<&str>,
  ) -> Result<Vec<PopulatedComment>, String>;
}

impl CommentOps for Client {
  async fn create_comment(
    &self,
    comment: BlogComment,
    reply_to: Option<&ObjectId>,
  ) -> Result<BlogComment, String> {
    create_comment(self, comment, reply_to)
      .await
      .map_err(|e| e.to_string())
  }
  async fn edit_comment(
    &self,
    id: &ObjectId,
    edit: CommentEdit,
  ) -> Result<Option<PopulatedComment>, String> {
    edit_comment(self, id, edit)
      .await
      .map_err(|e| e.to_string())
  }
  async fn like_comment(
    &self,
    id: &ObjectId,
  ) -> Result<Option<PopulatedComment>, String> {
    like_comment(self, id).await.map_err(|e| e.to_string())
  }
  async fn delete_comment(
    &self,
    id: &ObjectId,
  ) -> Result<(u64, Option<String>), String> {
    delete_comment(self, id).await.map_err(|e| e.to_string())
  }
  async fn list_comments(
    &self,
    path: &str,
    actor: Option<&str>,
  ) -> Result<Vec<PopulatedComment>, String> {
    list_comments(self, path, actor)
      .await
      .map_err(|e| e.to_string())
  }
}

/// Returns the timestamp string if it parses as valid RFC 3339, otherwise `None`.
fn validate_rfc3339(s: &str) -> Option<String> {
  DateTime::parse_from_rfc3339(s)
    .ok()
    .map(|dt| dt.to_rfc3339())
}

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
  let updated = collection
    .find_one_and_update(filter, update)
    .return_document(mongodb::options::ReturnDocument::After)
    .await?;
  match updated {
    Some(comment) => {
      Ok(Some(populate_replies(collection, comment, None).await?))
    }
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
  let created_time = validate_rfc3339(&comment.created_time)
    .unwrap_or_else(|| Utc::now().to_rfc3339());
  let comment = BlogComment {
    created_time,
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
  let mut set_fields = doc! { "edited_time": &now };
  if let Some(ref text) = edit.text {
    set_fields.insert("text", text);
  }
  if let Some(ref ct) = edit.created_time
    && let Some(valid) = validate_rfc3339(ct)
  {
    set_fields.insert("created_time", valid);
  }
  let update = doc! { "$set": set_fields };
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
pub async fn delete_comment(
  client: &Client,
  id: &ObjectId,
) -> Result<(u64, Option<String>), DbError> {
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
    .map(|reply_oid| {
      Box::pin(async move { delete_comment(client, &reply_oid).await })
    })
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
/// * `actor` — If provided, private comments by this author are included.
///   Private comments by other authors are filtered out.
///
/// # Behavior
///
/// Fetches every visible comment for the given `path` in a single Mongo
/// query, then assembles the reply forest in memory using each comment's
/// `reply_to` parent pointer. Public comments (where `is_private` is
/// `null` or `false`) are always returned. Private comments are only
/// returned if `actor` matches the comment's `author`. A comment whose
/// parent was filtered out (e.g. a public reply under a private parent)
/// becomes an orphan and is dropped.
///
/// # Returns
///
/// A `Vec<PopulatedComment>` of top-level comments with nested reply trees.
pub async fn list_comments(
  client: &Client,
  path: &str,
  actor: Option<&str>,
) -> Result<Vec<PopulatedComment>, DbError> {
  let collection = get_collection(client);

  let filter = match actor {
    Some(username) => doc! {
      "path": path,
      "$or": [
        { "is_private": { "$ne": true } },
        { "is_private": true, "author": username },
      ],
    },
    None => doc! {
      "path": path,
      "is_private": { "$ne": true },
    },
  };
  let all_comments: Vec<BlogComment> =
    collection.find(filter).await?.try_collect().await?;

  Ok(assemble_tree(all_comments))
}

/// Assembles a flat list of comments into a forest of populated trees.
///
/// Tree shape is reconstructed from each parent's `replies: Vec<String>`
/// array (parent → child pointers), matching the original recursive
/// behavior. A comment is a root iff (a) no other visible comment lists
/// it in its `replies` array, AND (b) its own `reply_to` is `None`. The
/// first condition handles legacy data where a child's `reply_to` is
/// missing or out of sync with the parent's `replies`. The second hides
/// children whose parent was filtered out by the privacy predicate.
/// Replies referenced but absent from the input are silently dropped.
fn assemble_tree(comments: Vec<BlogComment>) -> Vec<PopulatedComment> {
  let referenced: HashSet<String> = comments
    .iter()
    .flat_map(|c| c.replies.iter().cloned())
    .collect();

  let (roots, others): (Vec<BlogComment>, Vec<BlogComment>) =
    comments.into_iter().partition(|c| {
      c.reply_to.is_none()
        && c
          .id
          .as_ref()
          .is_none_or(|id| !referenced.contains(&id.to_hex()))
    });

  let mut by_id: HashMap<String, BlogComment> = others
    .into_iter()
    .filter_map(|c| c.id.map(|id| (id.to_hex(), c)))
    .collect();

  roots
    .into_iter()
    .map(|root| build_node(root, &mut by_id))
    .collect()
}

/// Recursively resolves a comment's reply tree by walking its `replies`
/// id list, draining `by_id` as it goes so each reply is moved exactly once.
fn build_node(
  mut comment: BlogComment,
  by_id: &mut HashMap<String, BlogComment>,
) -> PopulatedComment {
  let reply_ids = std::mem::take(&mut comment.replies);
  let children: Vec<BlogComment> = reply_ids
    .iter()
    .filter_map(|rid| by_id.remove(rid))
    .collect();
  let populated_replies: Vec<PopulatedComment> = children
    .into_iter()
    .map(|child| build_node(child, by_id))
    .collect();
  PopulatedComment::from_comment(comment, populated_replies)
}

/// Recursively fetches and nests a comment's reply tree.
///
/// Converts a [`BlogComment`] (which stores replies as a flat list of ID
/// strings) into a [`PopulatedComment`] with fully resolved, nested `replies`.
///
/// Private replies by other authors are filtered out.
///
/// # Arguments
///
/// * `collection` — Handle to the `comments` collection.
/// * `comment` — The comment whose replies should be populated.
/// * `actor` — If provided, private replies by this author are included.
///
/// # Returns
///
/// A [`PopulatedComment`] with its full reply tree resolved.
/// Uses `Box::pin` for async recursion.
fn populate_replies<'a>(
  collection: &'a mongodb::Collection<BlogComment>,
  comment: BlogComment,
  actor: Option<&'a str>,
) -> std::pin::Pin<
  Box<
    dyn std::future::Future<Output = Result<PopulatedComment, DbError>>
      + Send
      + 'a,
  >,
> {
  Box::pin(async move {
    if comment.replies.is_empty() {
      return Ok(PopulatedComment::from_comment(comment, vec![]));
    }

    let reply_oids = parse_oids(&comment.replies);
    let filter = doc! { "_id": { "$in": &reply_oids } };
    let reply_comments: Vec<BlogComment> =
      collection.find(filter).await?.try_collect().await?;

    // Filter out private replies by other authors
    let visible_replies: Vec<BlogComment> = reply_comments
      .into_iter()
      .filter(|reply| {
        if reply.is_private == Some(true) {
          actor.is_some_and(|a| a == reply.author)
        } else {
          true
        }
      })
      .collect();

    let futures: Vec<_> = visible_replies
      .into_iter()
      .map(|reply| populate_replies(collection, reply, actor))
      .collect();

    let mut populated_replies = Vec::with_capacity(futures.len());
    for result in futures::future::join_all(futures).await {
      populated_replies.push(result?);
    }

    Ok(PopulatedComment::from_comment(comment, populated_replies))
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn comment(
    text: &str,
    reply_to: Option<&ObjectId>,
    replies: &[&ObjectId],
  ) -> BlogComment {
    BlogComment {
      id: Some(ObjectId::new()),
      text: text.into(),
      markup: format!("<p>{text}</p>"),
      author: "alice".into(),
      created_time: "2025-01-01T00:00:00Z".into(),
      edited_time: None,
      path: "/blog/a".into(),
      likes: 0,
      is_private: None,
      reply_to: reply_to.map(|id| id.to_hex()),
      replies: replies.iter().map(|id| id.to_hex()).collect(),
    }
  }

  #[test]
  fn nests_via_parent_replies_array() {
    let parent = comment("parent", None, &[]);
    let child = comment("child", parent.id.as_ref(), &[]);
    let parent = BlogComment {
      replies: vec![child.id.unwrap().to_hex()],
      ..parent
    };

    let tree = assemble_tree(vec![parent, child]);
    assert_eq!(tree.len(), 1);
    assert_eq!(tree[0].text, "parent");
    assert_eq!(tree[0].replies.len(), 1);
    assert_eq!(tree[0].replies[0].text, "child");
  }

  // Regression: a child whose `reply_to` field is missing but is still
  // referenced by the parent's `replies` array must still be nested.
  // Earlier implementation that grouped by `reply_to` lost these.
  #[test]
  fn nests_when_child_reply_to_missing_but_parent_replies_lists_it() {
    let child = comment("child", None, &[]); // reply_to: None — inconsistent
    let parent = comment("parent", None, &[child.id.as_ref().unwrap()]);

    let tree = assemble_tree(vec![parent, child]);
    assert_eq!(tree.len(), 1, "only parent should be a root");
    assert_eq!(tree[0].text, "parent");
    assert_eq!(tree[0].replies.len(), 1);
    assert_eq!(tree[0].replies[0].text, "child");
  }

  #[test]
  fn nests_three_levels_deep() {
    let grandchild = comment("grandchild", None, &[]);
    let child = comment("child", None, &[grandchild.id.as_ref().unwrap()]);
    let parent = comment("parent", None, &[child.id.as_ref().unwrap()]);

    let tree = assemble_tree(vec![parent, child, grandchild]);
    assert_eq!(tree.len(), 1);
    assert_eq!(tree[0].replies.len(), 1);
    assert_eq!(tree[0].replies[0].replies.len(), 1);
    assert_eq!(tree[0].replies[0].replies[0].text, "grandchild");
  }

  #[test]
  fn drops_dangling_reply_id_silently() {
    let bogus_id = ObjectId::new();
    let parent = comment("parent", None, &[&bogus_id]);

    let tree = assemble_tree(vec![parent]);
    assert_eq!(tree.len(), 1);
    assert!(
      tree[0].replies.is_empty(),
      "child filtered out by privacy or missing should not break the parent",
    );
  }

  #[test]
  fn preserves_multiple_roots() {
    let a = comment("a", None, &[]);
    let b = comment("b", None, &[]);

    let tree = assemble_tree(vec![a, b]);
    assert_eq!(tree.len(), 2);
  }
}
