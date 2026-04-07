//! # Unified WebSocket Protocol
//!
//! Multiplexed protocol types for the `/api/connect` WebSocket endpoint.
//!
//! The client's active path (set by a `watch` scope request) drives
//! both comment event filtering and view-count update delivery.
//! There is no separate "views" request scope — view updates are
//! pushed automatically for the active path.

use super::{
  comments::{CommentEvent, CommentRequest, CommentResponse},
  health::{HealthDiagnostics, HealthOptions},
  opengraph::OpenGraphData,
  playback::{PlaybackRequest, PlaybackResponse},
  views::{ViewEvent, ViewsRequest, ViewsResponse},
};
use crate::AppState;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// The `"scope"` field in incoming WebSocket messages.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
  Health,
  OpenGraph,
  Playback,
  Views,
  Watch,
  #[default]
  Comments,
}

/// Request payload for a `watch` scope message.
///
/// Sets the client's active path for broadcast filtering
/// (comment events, view-count updates) without fetching data.
#[derive(Debug, Deserialize)]
pub struct WatchRequest {
  /// The page path to watch (e.g. `/blog/post-1`).
  pub path: String,
}

/// Request payload for an OpenGraph fetch over WebSocket.
#[derive(Debug, Deserialize)]
pub struct OpenGraphRequest {
  /// The URL to fetch OpenGraph data from.
  pub url: String,
}

/// Incoming WebSocket message from the client.
///
/// The `"scope"` field selects which subsystem handles the message.
/// If omitted it defaults to `"comments"`. View-count updates are
/// pushed automatically for the client's active path (set via the
/// `watch` scope).
///
/// # Examples
///
/// ```json
/// { "scope": "watch", "path": "/blog/post-1" }
/// { "scope": "comments", "action": "list", "path": "/blog/post-1" }
/// { "action": "create", "comment": { ... } }
/// ```
#[derive(Debug)]
pub enum ConnectRequest {
  /// A comment CRUD operation.
  Comments(Box<CommentRequest>),
  /// A page-view query (get / list).
  Views(ViewsRequest),
  /// A health-check diagnostics request.
  Health(HealthOptions),
  /// An OpenGraph metadata fetch.
  OpenGraph(OpenGraphRequest),
  /// A music playback query.
  Playback(PlaybackRequest),
  /// Sets the client's active path for broadcast filtering.
  Watch(WatchRequest),
}

impl ConnectRequest {
  /// Parses a JSON string into a [`ConnectRequest`].
  ///
  /// Extracts the `"scope"` field from the JSON payload and routes
  /// to the appropriate variant:
  ///
  /// | Scope        | Variant                        |
  /// |--------------|--------------------------------|
  /// | `"health"`   | [`Health`](Self::Health)       |
  /// | `"opengraph"`| [`OpenGraph`](Self::OpenGraph) |
  /// | `"views"`    | [`Views`](Self::Views)         |
  /// | `"watch"`    | [`Watch`](Self::Watch)         |
  /// | `"comments"` | [`Comments`](Self::Comments)   |
  ///
  /// When `"scope"` is omitted, defaults to `"comments"`.
  pub fn parse(text: &str) -> Result<Self, String> {
    let value: serde_json::Value =
      serde_json::from_str(text).map_err(|e| format!("invalid JSON: {e}"))?;

    // TODO: #2 revisit this
    let scope: Scope = value
      .get("scope")
      .map(|v| serde_json::from_value(v.clone()))
      .transpose()
      .map_err(|e| format!("invalid scope: {e}"))?
      .unwrap_or_default();

    match scope {
      Scope::Health => {
        let options: HealthOptions =
          serde_json::from_value(value).unwrap_or_default();
        Ok(ConnectRequest::Health(options))
      }
      Scope::OpenGraph => {
        let req: OpenGraphRequest = serde_json::from_value(value)
          .map_err(|e| format!("invalid opengraph request: {e}"))?;
        Ok(ConnectRequest::OpenGraph(req))
      }
      Scope::Views => {
        let req: ViewsRequest = serde_json::from_value(value)
          .map_err(|e| format!("invalid views request: {e}"))?;
        Ok(ConnectRequest::Views(req))
      }
      Scope::Playback => {
        let req: PlaybackRequest = serde_json::from_value(value)
          .map_err(|e| format!("invalid playback request: {e}"))?;
        Ok(ConnectRequest::Playback(req))
      }
      Scope::Watch => {
        let req: WatchRequest = serde_json::from_value(value)
          .map_err(|e| format!("invalid watch request: {e}"))?;
        Ok(ConnectRequest::Watch(req))
      }
      Scope::Comments => {
        let req: CommentRequest = serde_json::from_value(value)
          .map_err(|e| format!("invalid message: {e}"))?;
        Ok(ConnectRequest::Comments(Box::new(req)))
      }
    }
  }
}

/// Broadcast senders shared across all clients.
/// Used to publish events that every subscriber will receive.
pub struct EventSenders {
  pub comments: broadcast::Sender<CommentEvent>,
  pub views: broadcast::Sender<ViewEvent>,
  pub active_count: broadcast::Sender<usize>,
}

/// Per-client broadcast receivers.
/// Each client gets its own set of receivers upon connecting.
pub struct EventReceivers {
  pub comments: broadcast::Receiver<CommentEvent>,
  pub views: broadcast::Receiver<ViewEvent>,
  pub active_count: broadcast::Receiver<usize>,
}

/// Grouped sender/receiver handles for a single WebSocket client.
pub struct ClientChannels {
  pub senders: EventSenders,
  pub receivers: EventReceivers,
}

impl ClientChannels {
  /// Create a new set of channels for a client joining the WebSocket.
  ///
  /// Each receiver is subscribed from the corresponding sender on
  /// [`AppState`], so the client will receive all future broadcasts.
  pub fn from_app_state(state: &AppState) -> Self {
    // Cloning a `broadcast::Sender` is cheap
    // (Arc increment, no data copy).
    let senders = EventSenders {
      comments: state.comment_events.clone(),
      views: state.view_events.clone(),
      active_count: state.active_count_events.clone(),
    };
    let receivers = EventReceivers {
      comments: senders.comments.subscribe(),
      views: senders.views.subscribe(),
      active_count: senders.active_count.subscribe(),
    };
    Self { senders, receivers }
  }
}

/// Response payload for an OpenGraph fetch.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum OpenGraphResponse {
  /// Successful OpenGraph data extraction.
  Data(OpenGraphData),
  /// An error occurred while fetching OpenGraph data.
  Error { message: String },
}

/// Outgoing WebSocket message sent back to the client.
///
/// Discriminated by the `"scope"` JSON field (lowercased variant
/// name). Each variant wraps the scope-specific response payload.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "scope", rename_all = "lowercase")]
pub enum ConnectResponse {
  /// A comment-scoped response.
  Comments(CommentResponse),
  /// A view-scoped response.
  Views(ViewsResponse),
  /// A health-check diagnostics response.
  Health(HealthDiagnostics),
  /// An OpenGraph metadata response.
  OpenGraph(OpenGraphResponse),
  /// A playback-scoped response.
  Playback(PlaybackResponse),
  /// Acknowledgement of a watch scope request.
  Watch(WatchResponse),
}

/// Response payload for a `watch` scope request.
///
/// Confirms that the server has registered the client's active
/// path. Subsequent comment and view-count broadcast events will
/// be filtered to this path.
///
/// # Example
///
/// ```json
/// { "scope": "watch", "path": "/blog/post-1", "status": "success" }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct WatchResponse {
  /// The path that is now being watched.
  pub path: String,
  /// Always `"success"`.
  pub status: &'static str,
}
