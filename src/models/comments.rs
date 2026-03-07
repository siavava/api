use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

fn serialize_object_id<S: Serializer>(id: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error> {
  match id {
    Some(oid) => s.serialize_str(&oid.to_hex()),
    None => s.serialize_none(),
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogComment {
  #[serde(
    alias = "_id",
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id"
  )]
  pub id: Option<ObjectId>,
  pub text: String,
  pub markup: String,
  pub author: String,
  #[serde(default)]
  pub created_time: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub edited_time: Option<String>,
  pub path: String,
}

/// Partial update payload for editing a comment.
#[derive(Debug, Deserialize)]
pub struct CommentEdit {
  pub text: String,
}

/// WebSocket message types sent by the client.
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum WsRequest {
  Create {
    comment: BlogComment,
  },
  Edit {
    id: String,
    #[serde(flatten)]
    edit: CommentEdit,
  },
  Delete {
    id: String,
  },
  List {
    path: String,
  },
}

/// WebSocket message types sent back to the client.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WsResponse {
  Created { comment: BlogComment },
  Updated { comment: BlogComment },
  Deleted { id: String },
  List { comments: Vec<BlogComment> },
  Error { message: String },
}
