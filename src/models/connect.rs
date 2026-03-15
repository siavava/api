//! # Unified WebSocket Protocol
//!
//! Multiplexed protocol types for the `/api/connect` WebSocket endpoint.
//!
//! The client's active path (set by a comment `List` request) drives
//! both comment event filtering and view-count update delivery.
//! There is no separate "views" request scope — view updates are
//! pushed automatically for the active path.

use super::comments::{CommentRequest, CommentResponse};
use super::views::{ViewsRequest, ViewsResponse};
use serde::Serialize;

/// Incoming WebSocket message from the client.
///
/// Currently only comment operations are supported as requests.
/// View-count updates are pushed automatically for the active path
/// (set by a comment `List` request).
///
/// The `"scope"` field is accepted but optional; if omitted it
/// defaults to `"comments"`.
///
/// # Example
///
/// ```json
/// { "action": "list", "path": "/blog/post-1" }
/// { "scope": "comments", "action": "create", "comment": { ... } }
/// ```
#[derive(Debug)]
pub enum ConnectRequest {
  Comments(Box<CommentRequest>),
  Views(ViewsRequest),
}

impl ConnectRequest {
  /// Parses a JSON string into a `ConnectRequest`.
  ///
  /// Routes by the `"scope"` field: `"views"` goes to
  /// [`ViewsRequest`], anything else (including omitted) goes to
  /// [`CommentRequest`].
  pub fn parse(text: &str) -> Result<Self, String> {
    let value: serde_json::Value =
      serde_json::from_str(text).map_err(|e| format!("invalid JSON: {e}"))?;

    let scope = value
      .get("scope")
      .and_then(|v| v.as_str())
      .unwrap_or("comments");

    match scope {
      "views" => {
        let req: ViewsRequest =
          serde_json::from_value(value).map_err(|e| format!("invalid views request: {e}"))?;
        Ok(ConnectRequest::Views(req))
      }
      _ => {
        let req: CommentRequest =
          serde_json::from_value(value).map_err(|e| format!("invalid message: {e}"))?;
        Ok(ConnectRequest::Comments(Box::new(req)))
      }
    }
  }
}

/// Outgoing WebSocket message sent back to the client.
///
/// Discriminated by the `"scope"` JSON field.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "scope", rename_all = "lowercase")]
pub enum ConnectResponse {
  /// A comment-scoped response.
  Comments(CommentResponse),
  /// A view-scoped response.
  Views(ViewsResponse),
}
