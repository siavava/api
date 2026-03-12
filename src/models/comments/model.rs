//! Database and API-facing comment models.

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

/// Serializes an `Option<ObjectId>` as a hex string (or `null`).
fn serialize_object_id<S: Serializer>(
  id: &Option<ObjectId>,
  s: S,
) -> Result<S::Ok, S::Error> {
  match id {
    Some(oid) => s.serialize_str(&oid.to_hex()),
    None => s.serialize_none(),
  }
}

/// Serializes a `Vec<String>` of ObjectId hex strings as a JSON
/// array.
fn serialize_object_ids<S: Serializer>(
  ids: &[String],
  s: S,
) -> Result<S::Ok, S::Error> {
  s.collect_seq(ids.iter())
}

/// Database model for a comment, stored in the `comments` MongoDB
/// collection.
///
/// # Server-side Defaults
///
/// On creation, the following fields are overwritten with
/// server-side defaults regardless of client input:
/// - `created_time` — set to the current time.
/// - `edited_time` — set to `None`.
/// - `likes` — set to `0`.
/// - `replies` — set to an empty `Vec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogComment {
  /// MongoDB document ID.
  /// Aliased from `_id` on deserialization; serialized as a hex
  /// string. `None` for new (not-yet-inserted) comments.
  #[serde(
    alias = "_id",
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id"
  )]
  pub id: Option<ObjectId>,
  /// The raw text content of the comment (plain text).
  pub text: String,
  /// Pre-rendered markup version of the comment (e.g. HTML).
  pub markup: String,
  /// Display name of the comment author.
  pub author: String,
  /// ISO 8601 / RFC 3339 timestamp of when the comment was created.
  #[serde(default)]
  pub created_time: String,
  /// ISO 8601 / RFC 3339 timestamp of the last edit, if any.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub edited_time: Option<String>,
  /// The page path this comment belongs to
  /// (e.g. `/blog/some-post`).
  pub path: String,
  /// Number of likes.
  /// Incremented by
  /// [`like_comment`](crate::controllers::comments::like_comment);
  /// not idempotent.
  #[serde(default)]
  pub likes: i64,
  /// Whether this comment is private (visible only to its author).
  /// `None` or `false` means public.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_private: Option<bool>,
  /// Hex ObjectId of the parent comment, if this is a reply.
  /// `None` for top-level comments.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reply_to: Option<String>,
  /// Hex ObjectId strings of direct child replies.
  #[serde(default, serialize_with = "serialize_object_ids")]
  pub replies: Vec<String>,
}

/// API-facing comment with its reply tree fully resolved.
///
/// Same shape as [`BlogComment`] but `replies` contains nested
/// `PopulatedComment`s instead of flat ID strings.
#[derive(Debug, Clone, Serialize)]
pub struct PopulatedComment {
  /// MongoDB document ID, serialized as a hex string.
  #[serde(
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id"
  )]
  pub id: Option<ObjectId>,
  /// The raw text content of the comment.
  pub text: String,
  /// Pre-rendered markup version of the comment.
  pub markup: String,
  /// Display name of the comment author.
  pub author: String,
  /// ISO 8601 timestamp of creation.
  pub created_time: String,
  /// ISO 8601 timestamp of the last edit, if any.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub edited_time: Option<String>,
  /// The page path this comment belongs to.
  pub path: String,
  /// Number of likes.
  pub likes: i64,
  /// Whether this comment is private (visible only to its author).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_private: Option<bool>,
  /// Hex ObjectId of the parent comment, if this is a reply.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reply_to: Option<String>,
  /// Recursively populated child replies.
  pub replies: Vec<PopulatedComment>,
}

impl PopulatedComment {
  /// Converts a [`BlogComment`] into a `PopulatedComment`,
  /// attaching the already-resolved `populated_replies` as nested
  /// children.
  pub fn from_comment(
    comment: BlogComment,
    populated_replies: Vec<PopulatedComment>,
  ) -> Self {
    Self {
      id: comment.id,
      text: comment.text,
      markup: comment.markup,
      author: comment.author,
      created_time: comment.created_time,
      edited_time: comment.edited_time,
      path: comment.path,
      likes: comment.likes,
      is_private: comment.is_private,
      reply_to: comment.reply_to,
      replies: populated_replies,
    }
  }
}
