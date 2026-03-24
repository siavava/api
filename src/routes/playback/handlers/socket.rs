//! WebSocket action handlers for playback operations.

use crate::{
  controllers::playback::SpotifyClient,
  models::playback::{PlaybackRequest, PlaybackResponse},
};

/// Dispatches a pre-parsed [`PlaybackRequest`] to the matching handler.
pub async fn handle_request(
  spotify: &SpotifyClient,
  request: PlaybackRequest,
) -> PlaybackResponse {
  match request {
    PlaybackRequest::LastPlayed => handle_last_played(spotify).await,
    PlaybackRequest::Recents { limit } => {
      handle_recents(spotify, limit.unwrap_or(20)).await
    }
  }
}

/// Fetches the currently-playing or last-played track.
async fn handle_last_played(spotify: &SpotifyClient) -> PlaybackResponse {
  match spotify.last_played().await {
    Ok(track) => PlaybackResponse::LastPlayed { track },
    Err(e) => PlaybackResponse::Error {
      message: format!("failed to fetch last played: {e}"),
    },
  }
}

/// Fetches recently played tracks.
async fn handle_recents(
  spotify: &SpotifyClient,
  limit: u32,
) -> PlaybackResponse {
  match spotify.recents(limit).await {
    Ok(tracks) => PlaybackResponse::Recents { tracks },
    Err(e) => PlaybackResponse::Error {
      message: format!("failed to fetch recents: {e}"),
    },
  }
}
