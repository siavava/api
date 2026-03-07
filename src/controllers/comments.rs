use crate::models::comments::{BlogComment, CommentEdit};

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
) -> Result<BlogComment, DbError> {
  let collection = get_collection(client);
  let now = Utc::now().to_rfc3339();
  let comment = BlogComment {
    created_time: now,
    edited_time: None,
    ..comment
  };
  let result = collection.insert_one(&comment).await?;
  let id = result.inserted_id.as_object_id();
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
) -> Result<Vec<BlogComment>, DbError> {
  let collection = get_collection(client);
  let filter = doc! { "path": path };
  let cursor = collection.find(filter).await?;
  cursor.try_collect().await
}
