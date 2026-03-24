//! # Unified WebSocket Protocol
//!
//! Multiplexed protocol types for the `/api/connect` WebSocket endpoint.
//!
//! The client's active path (set by a comment `List` request) drives
//! both comment event filtering and view-count update delivery.
//! There is no separate "views" request scope — view updates are
//! pushed automatically for the active path.

use super::comments::{CommentEvent, CommentRequest, CommentResponse};
use super::health::{HealthDiagnostics, HealthOptions};
use super::opengraph::OpenGraphData;
use super::playback::{PlaybackRequest, PlaybackResponse};
use super::views::{ViewEvent, ViewsRequest, ViewsResponse};
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
  #[default]
  Comments,
}

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
/// Request payload for an OpenGraph fetch over WebSocket.
#[derive(Debug, Deserialize)]
pub struct OpenGraphRequest {
  /// The URL to fetch OpenGraph data from.
  pub url: String,
}

#[derive(Debug)]
pub enum ConnectRequest {
  Comments(Box<CommentRequest>),
  Views(ViewsRequest),
  Health(HealthOptions),
  OpenGraph(OpenGraphRequest),
  Playback(PlaybackRequest),
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
        let options: HealthOptions = serde_json::from_value(value).unwrap_or_default();
        Ok(ConnectRequest::Health(options))
      }
      Scope::OpenGraph => {
        let req: OpenGraphRequest =
          serde_json::from_value(value).map_err(|e| format!("invalid opengraph request: {e}"))?;
        Ok(ConnectRequest::OpenGraph(req))
      }
      Scope::Views => {
        let req: ViewsRequest =
          serde_json::from_value(value).map_err(|e| format!("invalid views request: {e}"))?;
        Ok(ConnectRequest::Views(req))
      }
      Scope::Playback => {
        let req: PlaybackRequest =
          serde_json::from_value(value).map_err(|e| format!("invalid playback request: {e}"))?;
        Ok(ConnectRequest::Playback(req))
      }
      Scope::Comments => {
        let req: CommentRequest =
          serde_json::from_value(value).map_err(|e| format!("invalid message: {e}"))?;
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

/// Outgoing WebSocket message sent back to the client.
///
/// Discriminated by the `"scope"` JSON field.
/// Response payload for an OpenGraph fetch.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum OpenGraphResponse {
  /// Successful OpenGraph data extraction.
  Data(OpenGraphData),
  /// An error occurred while fetching OpenGraph data.
  Error { message: String },
}

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
}
