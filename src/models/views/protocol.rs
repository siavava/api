//! WebSocket protocol types for view-count events.

use super::model::PageViews;
use serde::{Deserialize, Serialize};

/// Incoming view-count request.
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ViewsRequest {
  /// Fetch all view counts, optionally filtered to a route namespace
  /// prefix (e.g. `<p>`). Does not affect the active path.
  List {
    #[serde(default)]
    namespace: Option<String>,
  },
  /// Fetch the view count for a single route.
  Get { path: String },
}

/// View-count response types sent over the unified WebSocket.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ViewsResponse {
  /// All view counts, sorted by count descending.
  List { views: Vec<PageViews> },
  /// A view-count update event for a single route.
  Update {
    #[serde(flatten)]
    views: PageViews,
  },
  /// The current number of connected WebSocket clients.
  /// Broadcast to all clients whenever the count changes.
  #[serde(rename = "active-count")]
  ActiveCount { count: usize },
}

/// A view-count event broadcast to all connected WebSocket clients.
#[derive(Debug, Clone)]
pub struct ViewEvent {
  /// The updated page views data.
  pub views: PageViews,
}
