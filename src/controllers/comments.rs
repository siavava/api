use crate::models::comments::{BlogComment, CommentEdit, PopulatedComment};

use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{Client, bson::doc, bson::oid::ObjectId, error::Error as DbError};

const DB_NAME: &str = if cfg!(debug_assertions) {
  "feed-dev"
} else {
  "feed"
};

const COLL_NAME: &str = "comments";

pub fn get_collection(client: &Client) -> mongodb::Collection<BlogComment> {
  client
    .database(DB_NAME)
    .collection::<BlogComment>(COLL_NAME)
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

  Ok(BlogComment {
    id,
    ..comment
  })
}

pub async fn edit_comment(
  client: &Client,
  id: &ObjectId,
  edit: CommentEdit,
) -> Result<Option<BlogComment>, DbError> {
  let collection = get_collection(client);
  let filter = doc! { "_id": id };
  let now = Utc::now().to_rfc3339();
  let update = doc! {
    "$set": {
      "text": &edit.text,
      "edited_time": &now,
    }
  };

  collection.update_one(filter.clone(), update).await?;
  let updated = collection.find_one(filter).await?;
  Ok(updated)
}

pub async fn delete_comment(client: &Client, id: &ObjectId) -> Result<bool, DbError> {
  let collection = get_collection(client);
  let filter = doc! { "_id": id };
  let result = collection.delete_one(filter).await?;
  Ok(result.deleted_count > 0)
}

pub async fn list_comments(
  client: &Client,
  path: &str,
) -> Result<Vec<PopulatedComment>, DbError> {
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

  let reply_oids: Vec<ObjectId> = comment
    .replies
    .iter()
    .filter_map(|id_str| ObjectId::parse_str(id_str).ok())
    .collect();

  let filter = doc! { "_id": { "$in": &reply_oids } };
  let reply_comments: Vec<BlogComment> = collection.find(filter).await?.try_collect().await?;

  let mut populated_replies = Vec::with_capacity(reply_comments.len());
  for reply in reply_comments {
    let p = Box::pin(populate_replies(collection.clone(), reply)).await?;
    populated_replies.push(p);
  }

  Ok(PopulatedComment::from_comment(comment, populated_replies))
}
