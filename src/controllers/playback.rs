//! Spotify Web API client for playback data.
//!
//! Handles OAuth token refresh and exposes methods to fetch the
//! currently-playing track and recently played history.

use crate::models::playback::Track;
use parking_lot::Mutex;
use serde::Deserialize;
use std::time::{Duration, Instant};
use tracing::{error, info};

/// Cached OAuth access token with expiry.
struct CachedToken {
  access_token: String,
  expires_at: Instant,
}

/// Spotify API client with automatic token refresh.
pub struct SpotifyClient {
  http: reqwest::Client,
  client_id: String,
  client_secret: String,
  refresh_token: String,
  token: Mutex<Option<CachedToken>>,
}

#[derive(Deserialize)]
struct TokenResponse {
  access_token: String,
  expires_in: u64,
}

/// Spotify's currently-playing endpoint response.
#[derive(Deserialize)]
struct CurrentlyPlayingResponse {
  is_playing: bool,
  progress_ms: Option<u64>,
  item: Option<SpotifyTrack>,
}

/// Spotify's recently-played endpoint response.
#[derive(Deserialize)]
struct RecentlyPlayedResponse {
  items: Vec<PlayHistoryItem>,
}

#[derive(Deserialize)]
struct PlayHistoryItem {
  track: SpotifyTrack,
  played_at: String,
}

#[derive(Deserialize)]
struct SpotifyTrack {
  id: Option<String>,
  name: String,
  artists: Vec<SpotifyArtist>,
  album: SpotifyAlbum,
  external_urls: Option<ExternalUrls>,
  duration_ms: u64,
}

#[derive(Deserialize)]
struct SpotifyArtist {
  name: String,
}

#[derive(Deserialize, Default)]
struct SpotifyAlbum {
  name: String,
  images: Vec<SpotifyImage>,
}

#[derive(Deserialize)]
struct SpotifyImage {
  url: String,
}

#[derive(Deserialize)]
struct ExternalUrls {
  spotify: Option<String>,
}

impl SpotifyClient {
  /// Creates a new client from environment credentials.
  ///
  /// Returns `None` if any required env var is missing.
  pub fn from_env() -> Option<Self> {
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").ok()?;
    let client_secret = std::env::var("SPOTIFY_CLIENT_SECRET").ok()?;
    let refresh_token = std::env::var("SPOTIFY_REFRESH_TOKEN").ok()?;

    info!("spotify client initialized");

    Some(Self {
      http: reqwest::Client::new(),
      client_id,
      client_secret,
      refresh_token,
      token: Mutex::new(None),
    })
  }

  /// Returns a valid access token, refreshing if needed.
  async fn access_token(&self) -> Result<String, String> {
    // Check cached token.
    {
      let guard = self.token.lock();
      if let Some(ref cached) = *guard
        && Instant::now() < cached.expires_at
      {
        return Ok(cached.access_token.clone());
      }
    }

    // Refresh the token.
    let resp = self
      .http
      .post("https://accounts.spotify.com/api/token")
      .basic_auth(&self.client_id, Some(&self.client_secret))
      .form(&[
        ("grant_type", "refresh_token"),
        ("refresh_token", &self.refresh_token),
      ])
      .send()
      .await
      .map_err(|e| format!("token refresh request failed: {e}"))?;

    if !resp.status().is_success() {
      let status = resp.status();
      let body = resp.text().await.unwrap_or_default();
      error!("spotify token refresh failed: {status} {body}");
      return Err(format!("token refresh failed: {status}"));
    }

    let token_resp: TokenResponse = resp
      .json()
      .await
      .map_err(|e| format!("failed to parse token response: {e}"))?;

    let access_token = token_resp.access_token.clone();
    let expires_at =
      Instant::now() + Duration::from_secs(token_resp.expires_in.saturating_sub(60));

    *self.token.lock() = Some(CachedToken {
      access_token: token_resp.access_token,
      expires_at,
    });

    Ok(access_token)
  }

  /// Fetches the currently-playing or last-played track.
  pub async fn last_played(&self) -> Result<Option<Track>, String> {
    let token = self.access_token().await?;

    let resp = self
      .http
      .get("https://api.spotify.com/v1/me/player/currently-playing")
      .bearer_auth(&token)
      .send()
      .await
      .map_err(|e| format!("spotify api request failed: {e}"))?;

    // 204 = nothing is actively playing. Fall back to most recent track.
    if resp.status().as_u16() == 204 {
      return self.recents(1).await.map(|mut v| v.pop());
    }

    if !resp.status().is_success() {
      let status = resp.status();
      return Err(format!("spotify api error: {status}"));
    }

    let body: CurrentlyPlayingResponse = resp
      .json()
      .await
      .map_err(|e| format!("failed to parse currently-playing: {e}"))?;

    match body.item {
      Some(item) => {
        let preview = match item.id.as_deref() {
          Some(id) => {
            let previews = fetch_preview_urls(&self.http, &[id]).await;
            previews.into_values().next()
          }
          None => None,
        };
        Ok(Some(to_track(&item, body.is_playing, body.progress_ms, None, preview.as_deref())))
      }
      None => Ok(None),
    }
  }

  /// Fetches recently played tracks, deduplicated by track URL.
  ///
  /// Spotify's API returns one entry per play, so a song on repeat
  /// appears many times. We keep only the most recent play of each
  /// unique track and return up to `limit` results.
  pub async fn recents(&self, limit: u32) -> Result<Vec<Track>, String> {
    let token = self.access_token().await?;
    // Fetch the max (50) to have enough unique tracks after dedup.
    let fetch_limit = 50u32;

    let resp = self
      .http
      .get("https://api.spotify.com/v1/me/player/recently-played")
      .bearer_auth(&token)
      .query(&[("limit", fetch_limit.to_string())])
      .send()
      .await
      .map_err(|e| format!("spotify api request failed: {e}"))?;

    if !resp.status().is_success() {
      let status = resp.status();
      return Err(format!("spotify api error: {status}"));
    }

    let body: RecentlyPlayedResponse = resp
      .json()
      .await
      .map_err(|e| format!("failed to parse recently-played: {e}"))?;

    // Dedup by track name + artist, keeping the first (most recent) occurrence.
    let limit = limit.clamp(1, 50) as usize;
    let mut seen = std::collections::HashSet::new();
    let deduped: Vec<&PlayHistoryItem> = body
      .items
      .iter()
      .filter(|item| {
        seen.insert((
          &item.track.name,
          item.track.artists.iter().map(|a| &a.name).collect::<Vec<_>>(),
        ))
      })
      .take(limit)
      .collect();

    // Fetch preview URLs concurrently from embed pages.
    let ids: Vec<&str> = deduped
      .iter()
      .filter_map(|item| item.track.id.as_deref())
      .collect();
    let previews = fetch_preview_urls(&self.http, &ids).await;

    let tracks: Vec<Track> = deduped
      .iter()
      .map(|item| {
        let preview = item.track.id.as_deref().and_then(|id| previews.get(id).map(|s| s.as_str()));
        to_track(&item.track, false, None, Some(&item.played_at), preview)
      })
      .collect();

    Ok(tracks)
  }
}

/// Fetches preview URLs for tracks by scraping Spotify embed pages.
///
/// Spotify deprecated `preview_url` in their API, but the embed
/// pages (`/embed/track/{id}`) still contain the preview URL in
/// inline JSON under `audioPreview.url`.
///
/// Fetches are done concurrently for all tracks.
async fn fetch_preview_urls(
  http: &reqwest::Client,
  track_ids: &[&str],
) -> std::collections::HashMap<String, String> {
  let futures: Vec<_> = track_ids
    .iter()
    .map(|id| {
      let id = id.to_string();
      let http = http.clone();
      async move {
        let url = format!("https://open.spotify.com/embed/track/{id}");
        let preview = fetch_embed_preview(&http, &url).await;
        (id, preview)
      }
    })
    .collect();

  let results = futures_util::future::join_all(futures).await;

  results
    .into_iter()
    .filter_map(|(id, preview)| Some((id, preview?)))
    .collect()
}

/// Scrapes a single Spotify embed page for an `audioPreview.url`.
async fn fetch_embed_preview(http: &reqwest::Client, url: &str) -> Option<String> {
  let html = http.get(url).send().await.ok()?.text().await.ok()?;

  // The embed page contains inline JSON with an "audioPreview" object.
  // Look for the pattern: "audioPreview":{"url":"<preview_url>"}
  let marker = "\"audioPreview\":{\"url\":\"";
  let start = html.find(marker)? + marker.len();
  let rest = &html[start..];
  let end = rest.find('"')?;
  Some(rest[..end].to_string())
}

/// Converts a Spotify track object into our domain [`Track`].
fn to_track(
  t: &SpotifyTrack,
  is_playing: bool,
  progress_ms: Option<u64>,
  played_at: Option<&str>,
  preview_url: Option<&str>,
) -> Track {
  Track {
    name: t.name.clone(),
    artist: t
      .artists
      .iter()
      .map(|a| a.name.as_str())
      .collect::<Vec<_>>()
      .join(", "),
    album: t.album.name.clone(),
    album_art: t.album.images.first().map(|i| i.url.clone()),
    url: t.external_urls.as_ref().and_then(|u| u.spotify.clone()),
    preview_url: preview_url.map(String::from),
    duration_ms: t.duration_ms,
    progress_ms,
    is_playing,
    played_at: played_at.map(String::from),
  }
}
