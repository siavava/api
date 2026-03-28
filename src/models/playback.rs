//! Data models and WebSocket protocol types for Spotify playback.

use serde::{Deserialize, Serialize};

/// A simplified Spotify track.
#[derive(Debug, Clone, Serialize)]
pub struct Track {
  pub name: String,
  pub artist: String,
  pub album: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub album_art: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub preview_url: Option<String>,
  pub duration_ms: u64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub progress_ms: Option<u64>,
  pub is_playing: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub played_at: Option<String>,
}

/// Incoming WebSocket message from the client.
///
/// Discriminated by the `"action"` JSON field.
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum PlaybackRequest {
  /// Get the currently-playing or last-played track.
  LastPlayed,
  /// Get a list of recently played tracks.
  Recents {
    /// Maximum number of tracks to return (default 20, max 50).
    #[serde(default)]
    limit: Option<u32>,
  },
}

/// Outgoing WebSocket message sent back to the client.
///
/// Discriminated by the `"type"` JSON field.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum PlaybackResponse {
  /// The currently-playing or last-played track.
  LastPlayed {
    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<Track>,
  },
  /// A list of recently played tracks.
  Recents { tracks: Vec<Track> },
  /// An error message.
  Error { message: String },
}

#[cfg(test)]
mod tests;
