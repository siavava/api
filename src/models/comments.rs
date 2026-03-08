//! # Comment Models
//!
//! Data models for blog comments, replies, and the WebSocket protocol used
//! for real-time comment operations.
//!
//! # Exports
//!
//! * [`BlogComment`] — Database model for a single comment.
//! * [`PopulatedComment`] — API-facing comment with its reply tree resolved.
//! * [`CommentEdit`] — Partial update payload for editing a comment.
//! * [`CommentRequest`] — Incoming WebSocket message (tagged by `"action"`).
//! * [`CommentResponse`] — Outgoing WebSocket message (tagged by `"type"`).

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

/// A comment event broadcast to all connected WebSocket clients.
///
/// Carries the page path the event applies to and the response payload.
#[derive(Debug, Clone)]
pub struct CommentEvent {
  /// The page path the affected comment belongs to.
  pub path: String,
  /// The response payload to forward to subscribed clients.
  pub response: CommentResponse,
}

/// Serializes an `Option<ObjectId>` as a hex string (or `null`).
///
/// # Arguments
///
/// * `id` — The optional ObjectId to serialize.
/// * `s` — The serde serializer.
///
/// # Returns
///
/// `Ok(S::Ok)` after writing a hex string or `null`.
fn serialize_object_id<S: Serializer>(id: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error> {
  match id {
    Some(oid) => s.serialize_str(&oid.to_hex()),
    None => s.serialize_none(),
  }
}

/// Serializes a `Vec<String>` of ObjectId hex strings as a JSON array.
///
/// # Arguments
///
/// * `ids` — The vector of hex-encoded ObjectId strings.
/// * `s` — The serde serializer.
///
/// # Returns
///
/// `Ok(S::Ok)` after writing a JSON array of strings.
fn serialize_object_ids<S: Serializer>(ids: &[String], s: S) -> Result<S::Ok, S::Error> {
  s.collect_seq(ids.iter())
}

/// Database model for a comment, stored in the `comments` MongoDB collection.
///
/// # Server-side Defaults
///
/// On creation, the following fields are overwritten with server-side defaults
/// regardless of client input:
/// - `created_time` — set to the current time.
/// - `edited_time` — set to `None`.
/// - `likes` — set to `0`.
/// - `replies` — set to an empty `Vec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogComment {
  /// MongoDB document ID.
  /// Aliased from `_id` on deserialization; serialized as a hex string.
  /// `None` for new (not-yet-inserted) comments.
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
  /// The page path this comment belongs to (e.g. `/blog/some-post`).
  pub path: String,
  /// Number of likes.
  /// Incremented by [`like_comment`](crate::controllers::comments::like_comment);
  /// not idempotent.
  #[serde(default)]
  pub likes: i64,
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
  /// Hex ObjectId of the parent comment, if this is a reply.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reply_to: Option<String>,
  /// Recursively populated child replies.
  pub replies: Vec<PopulatedComment>,
}

impl PopulatedComment {
  /// Converts a [`BlogComment`] into a `PopulatedComment`, attaching the
  /// already-resolved `populated_replies` as nested children.
  ///
  /// # Arguments
  ///
  /// * `comment` — The source comment.
  /// * `populated_replies` — Pre-resolved child replies.
  ///
  /// # Returns
  ///
  /// A `PopulatedComment` with fields copied from `comment` and the given
  /// `populated_replies` attached as nested children.
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
  /// The new text content to replace the existing comment text.
  pub text: String,
}

/// Incoming WebSocket message from the client.
///
/// Discriminated by the `"action"` JSON field (lowercased variant name).
///
/// # Variants
///
/// | Action   | Description                                                        |
/// |----------|--------------------------------------------------------------------|
/// | `Create` | Create a new comment, optionally as a reply to `reply_to`.         |
/// | `Edit`   | Update a comment's text. [`CommentEdit`] fields are flattened into the top level (i.e. `{ "action": "edit", "id": "...", "text": "..." }`). |
/// | `Like`   | Increment a comment's like count by 1.                             |
/// | `Delete` | Delete a comment and all its nested replies.                       |
/// | `List`   | List all top-level comments (with populated replies) for a path.   |
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum CommentRequest {
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

/// Outgoing WebSocket message sent back to the client.
///
/// Discriminated by the `"type"` JSON field (lowercased variant name).
///
/// # Variants
///
/// | Type      | Description                                                       |
/// |-----------|-------------------------------------------------------------------|
/// | `Created` | Echoes back the newly inserted comment (without populated replies). |
/// | `Updated` | The edited comment with its full reply tree.                      |
/// | `Liked`   | The liked comment with its full reply tree.                       |
/// | `Deleted` | Confirms deletion with the original `id` and `deleted_count` (total documents removed, including nested replies). |
/// | `List`    | All top-level comments for the requested path, with nested replies. |
/// | `Error`   | A human-readable error message (invalid input, not found, DB error, etc.). |
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CommentResponse {
  Created { comment: BlogComment },
  Updated { comment: PopulatedComment },
  Liked { comment: PopulatedComment },
  Deleted { id: String, deleted_count: u64 },
  List { comments: Vec<PopulatedComment> },
  Error { message: String },
}
