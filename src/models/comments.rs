use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

fn serialize_object_id<S: Serializer>(id: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error> {
  match id {
    Some(oid) => s.serialize_str(&oid.to_hex()),
    None => s.serialize_none(),
  }
}

fn serialize_object_ids<S: Serializer>(ids: &Vec<String>, s: S) -> Result<S::Ok, S::Error> {
  s.collect_seq(ids.iter())
}

/// DB model for a comment.
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
  #[serde(default)]
  pub likes: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reply_to: Option<String>,
  #[serde(default, serialize_with = "serialize_object_ids")]
  pub replies: Vec<String>,
}

/// Response model with populated replies.
#[derive(Debug, Clone, Serialize)]
pub struct PopulatedComment {
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id"
  )]
  pub id: Option<ObjectId>,
  pub text: String,
  pub markup: String,
  pub author: String,
  pub created_time: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub edited_time: Option<String>,
  pub path: String,
  pub likes: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reply_to: Option<String>,
  pub replies: Vec<PopulatedComment>,
}

impl PopulatedComment {
  pub fn from_comment(comment: BlogComment, populated_replies: Vec<PopulatedComment>) -> Self {
    Self {
      id: comment.id,
      text: comment.text,
      markup: comment.markup,
      author: comment.author,
      created_time: comment.created_time,
      edited_time: comment.edited_time,
      path: comment.path,
      likes: comment.likes,
      reply_to: comment.reply_to,
      replies: populated_replies,
    }
  }
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
    reply_to: Option<String>,
  },
  Edit {
    id: String,
    #[serde(flatten)]
    edit: CommentEdit,
  },
  Like {
    id: String,
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
  Updated { comment: PopulatedComment },
  Liked { comment: PopulatedComment },
  Deleted { id: String, deleted_count: u64 },
  List { comments: Vec<PopulatedComment> },
  Error { message: String },
}
