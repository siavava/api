use crate::models::comments::{BlogComment, CommentEdit, PopulatedComment};

use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{Client, bson::doc, bson::oid::ObjectId, error::Error as DbError};

const COLL_NAME: &str = "comments";

fn get_collection(client: &Client) -> mongodb::Collection<BlogComment> {
  crate::db::collection(client, COLL_NAME)
}

/// Parse a list of hex ObjectId strings into ObjectIds, skipping invalid ones.
fn parse_oids(ids: &[String]) -> Vec<ObjectId> {
  ids
    .iter()
    .filter_map(|s| ObjectId::parse_str(s).ok())
    .collect()
}

/// Update a comment by ID then return it with populated replies.
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

pub async fn like_comment(
  client: &Client,
  id: &ObjectId,
) -> Result<Option<PopulatedComment>, DbError> {
  let collection = get_collection(client);
  let update = doc! { "$inc": { "likes": 1 } };
  update_and_populate(&collection, id, update).await
}

pub async fn delete_comment(client: &Client, id: &ObjectId) -> Result<u64, DbError> {
  let collection = get_collection(client);

  // Find the comment first so we can access its replies and parent
  let filter = doc! { "_id": id };
  let comment = match collection.find_one(filter.clone()).await? {
    Some(c) => c,
    None => return Ok(0),
  };

  // Recursively delete all replies concurrently
  let reply_futures: Vec<_> = parse_oids(&comment.replies)
    .into_iter()
    .map(|reply_oid| Box::pin(async move { delete_comment(client, &reply_oid).await }))
    .collect();

  let mut deleted_count: u64 = 0;
  for result in futures::future::join_all(reply_futures).await {
    deleted_count += result?;
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
  Ok(deleted_count + result.deleted_count)
}

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

  let mut populated_replies = Vec::with_capacity(reply_comments.len());
  for reply in reply_comments {
    let p = Box::pin(populate_replies(collection.clone(), reply)).await?;
    populated_replies.push(p);
  }

  Ok(PopulatedComment::from_comment(comment, populated_replies))
}
