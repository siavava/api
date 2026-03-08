//! Per-action handlers for comment WebSocket requests.

use crate::{
  controllers::comments,
  models::comments::{BlogComment, CommentEdit, CommentRequest, CommentResponse},
};
use mongodb::bson::oid::ObjectId;

/// Response paired with an optional broadcast path.
///
/// - `Some(path)` — mutation succeeded; broadcast to clients on
///   `path`.
/// - `None` — no broadcast (list response or error).
pub type Handled = (CommentResponse, Option<String>);

/// Helper to build an error [`Handled`] tuple (never broadcast).
fn err(message: impl Into<String>) -> Handled {
  (CommentResponse::Error { message: message.into() }, None)
}

/// Parses a hex string as an `ObjectId`.
fn parse_oid(id: &str) -> Result<ObjectId, String> {
  ObjectId::parse_str(id).map_err(|e| format!("invalid id: {e}"))
}

/// Parses a raw WebSocket text frame and dispatches it to the
/// matching handler.
pub async fn handle_message(
  db: &mongodb::Client,
  text: &str,
  active_route: &mut Option<String>,
) -> Handled {
  let request: CommentRequest = match serde_json::from_str(text) {
    Ok(req) => req,
    Err(e) => return err(format!("invalid message: {e}")),
  };

  match request {
    CommentRequest::Create { comment, reply_to } =>
      handle_create(db, comment, reply_to).await,
    CommentRequest::Edit { id, edit } =>
      handle_edit(db, id, edit).await,
    CommentRequest::Like { id } =>
      handle_like(db, id).await,
    CommentRequest::Delete { id } =>
      handle_delete(db, id).await,
    CommentRequest::List { path } => {
      *active_route = Some(path.clone());
      handle_list(db, path).await
    }
  }
}

/// Creates a new comment (optionally as a reply).
async fn handle_create(
  db: &mongodb::Client,
  comment: BlogComment,
  reply_to: Option<String>,
) -> Handled {
  let path = comment.path.clone();
  let parent_oid = match reply_to {
    Some(ref id_str) => match parse_oid(id_str) {
      Ok(oid) => Some(oid),
      Err(e) => return err(e),
    },
    None => None,
  };

  match comments::create_comment(db, comment, parent_oid.as_ref()).await {
    Ok(created) => (
      CommentResponse::Created { comment: created },
      Some(path),
    ),
    Err(e) => err(format!("failed to create comment: {e}")),
  }
}

/// Edits an existing comment's text.
async fn handle_edit(
  db: &mongodb::Client,
  id: String,
  edit: CommentEdit,
) -> Handled {
  let oid = match parse_oid(&id) {
    Ok(oid) => oid,
    Err(e) => return err(e),
  };

  match comments::edit_comment(db, &oid, edit).await {
    Ok(Some(updated)) => {
      let path = updated.path.clone();
      (CommentResponse::Updated { comment: updated }, Some(path))
    }
    Ok(None) => err("comment not found"),
    Err(e) => err(format!("failed to edit comment: {e}")),
  }
}

/// Increments a comment's like count.
async fn handle_like(
  db: &mongodb::Client,
  id: String,
) -> Handled {
  let oid = match parse_oid(&id) {
    Ok(oid) => oid,
    Err(e) => return err(e),
  };

  match comments::like_comment(db, &oid).await {
    Ok(Some(liked)) => {
      let path = liked.path.clone();
      (CommentResponse::Liked { comment: liked }, Some(path))
    }
    Ok(None) => err("comment not found"),
    Err(e) => err(format!("failed to like comment: {e}")),
  }
}

/// Deletes a comment and all its nested replies.
async fn handle_delete(
  db: &mongodb::Client,
  id: String,
) -> Handled {
  let oid = match parse_oid(&id) {
    Ok(oid) => oid,
    Err(e) => return err(e),
  };

  match comments::delete_comment(db, &oid).await {
    Ok((count, Some(path))) if count > 0 => (
      CommentResponse::Deleted { id, deleted_count: count },
      Some(path),
    ),
    Ok(_) => err("comment not found"),
    Err(e) => err(format!("failed to delete comment: {e}")),
  }
}

/// Lists all top-level comments for a page path.
async fn handle_list(
  db: &mongodb::Client,
  path: String,
) -> Handled {
  match comments::list_comments(db, &path).await {
    Ok(list) => (CommentResponse::List { comments: list }, None),
    Err(e) => err(format!("failed to list comments: {e}")),
  }
}
