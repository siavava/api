//! WebSocket protocol types for the comments endpoint.

use super::model::{BlogComment, PopulatedComment};
use serde::{Deserialize, Serialize};

/// Partial update payload for editing a comment.
#[derive(Debug, Deserialize)]
pub struct CommentEdit {
  /// The new text content to replace the existing comment text.
  pub text: String,
}

/// Incoming WebSocket message from the client.
///
/// Discriminated by the `"action"` JSON field (lowercased variant
/// name).
///
/// # Variants
///
/// | Action   | Description                              |
/// |----------|------------------------------------------|
/// | `Create` | Create a new comment (optionally reply).  |
/// | `Edit`   | Update a comment's text.                 |
/// | `Like`   | Increment a comment's like count by 1.   |
/// | `Delete` | Delete a comment and all nested replies.  |
/// | `List`   | List top-level comments for a path.      |
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
/// Discriminated by the `"type"` JSON field (lowercased variant
/// name).
///
/// # Variants
///
/// | Type      | Description                              |
/// |-----------|------------------------------------------|
/// | `Created` | The newly inserted comment.              |
/// | `Updated` | The edited comment with full reply tree.  |
/// | `Liked`   | The liked comment with full reply tree.   |
/// | `Deleted` | Confirmation with `id` and count.        |
/// | `List`    | Top-level comments with nested replies.  |
/// | `Error`   | A human-readable error message.          |
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

/// A comment event broadcast to all connected WebSocket clients.
///
/// Carries the page path the event applies to and the response
/// payload.
#[derive(Debug, Clone)]
pub struct CommentEvent {
  /// The page path the affected comment belongs to.
  pub path: String,
  /// The response payload to forward to subscribed clients.
  pub response: CommentResponse,
}
