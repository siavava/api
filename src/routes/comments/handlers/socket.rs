//! WebSocket action handlers for comment operations.

use crate::{
  controllers::comments::CommentOps,
  db::parse_oid,
  models::comments::{
    BlogComment, CommentEdit, CommentRequest, CommentResponse,
  },
};

/// Response paired with an optional broadcast path.
///
/// - `Some(path)` — mutation succeeded; broadcast to clients on
///   `path`.
/// - `None` — no broadcast (list response or error).
pub type Handled = (CommentResponse, Option<String>);

/// Helper to build an error [`Handled`] tuple (never broadcast).
fn err(message: impl Into<String>) -> Handled {
  (
    CommentResponse::Error {
      message: message.into(),
    },
    None,
  )
}

/// Parses a raw WebSocket text frame and dispatches it to the
/// matching handler.
pub async fn handle_message(
  db: &impl CommentOps,
  text: &str,
  active_route: &mut Option<String>,
) -> Handled {
  let request: CommentRequest = match serde_json::from_str(text) {
    Ok(req) => req,
    Err(e) => return err(format!("invalid message: {e}")),
  };

  handle_request(db, request, active_route).await
}

/// Dispatches a pre-parsed [`CommentRequest`] to the matching handler.
pub async fn handle_request(
  db: &impl CommentOps,
  request: CommentRequest,
  active_route: &mut Option<String>,
) -> Handled {
  match request {
    CommentRequest::Create { comment, reply_to } => {
      handle_create(db, comment, reply_to).await
    }
    CommentRequest::Edit { id, edit } => handle_edit(db, id, edit).await,
    CommentRequest::Like { id } => handle_like(db, id).await,
    CommentRequest::Delete { id } => handle_delete(db, id).await,
    CommentRequest::List { path, actor } => {
      *active_route = Some(path.clone());
      handle_list(db, path, actor).await
    }
  }
}

/// Creates a new comment (optionally as a reply).
async fn handle_create(
  db: &impl CommentOps,
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

  match db.create_comment(comment, parent_oid.as_ref()).await {
    Ok(created) => (CommentResponse::Created { comment: created }, Some(path)),
    Err(e) => err(format!("failed to create comment: {e}")),
  }
}

/// Edits an existing comment's text.
async fn handle_edit(
  db: &impl CommentOps,
  id: String,
  edit: CommentEdit,
) -> Handled {
  let oid = match parse_oid(&id) {
    Ok(oid) => oid,
    Err(e) => return err(e),
  };

  match db.edit_comment(&oid, edit).await {
    Ok(Some(updated)) => {
      let path = updated.path.clone();
      (CommentResponse::Updated { comment: updated }, Some(path))
    }
    Ok(None) => err("comment not found"),
    Err(e) => err(format!("failed to edit comment: {e}")),
  }
}

/// Increments a comment's like count.
async fn handle_like(db: &impl CommentOps, id: String) -> Handled {
  let oid = match parse_oid(&id) {
    Ok(oid) => oid,
    Err(e) => return err(e),
  };

  match db.like_comment(&oid).await {
    Ok(Some(liked)) => {
      let path = liked.path.clone();
      (CommentResponse::Liked { comment: liked }, Some(path))
    }
    Ok(None) => err("comment not found"),
    Err(e) => err(format!("failed to like comment: {e}")),
  }
}

/// Deletes a comment and all its nested replies.
async fn handle_delete(db: &impl CommentOps, id: String) -> Handled {
  let oid = match parse_oid(&id) {
    Ok(oid) => oid,
    Err(e) => return err(e),
  };

  match db.delete_comment(&oid).await {
    Ok((count, Some(path))) if count > 0 => (
      CommentResponse::Deleted {
        id,
        deleted_count: count,
      },
      Some(path),
    ),
    Ok(_) => err("comment not found"),
    Err(e) => err(format!("failed to delete comment: {e}")),
  }
}

/// Lists all top-level comments for a page path.
async fn handle_list(
  db: &impl CommentOps,
  path: String,
  actor: Option<String>,
) -> Handled {
  match db.list_comments(&path, actor.as_deref()).await {
    Ok(list) => (CommentResponse::List { comments: list }, None),
    Err(e) => err(format!("failed to list comments: {e}")),
  }
}

#[cfg(test)]
mod tests;
