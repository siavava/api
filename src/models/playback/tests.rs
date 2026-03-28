//! Tests for playback model request/response
//! serialization and optional field handling.

use super::*;

fn sample_track() -> Track {
  Track {
    name: "Song".into(),
    artist: "Artist".into(),
    album: "Album".into(),
    album_art: None,
    url: None,
    preview_url: None,
    duration_ms: 200000,
    progress_ms: None,
    is_playing: false,
    played_at: None,
  }
}

#[test]
fn deserialize_last_played_request() {
  let json = r#"{"action": "last-played"}"#;
  let req: PlaybackRequest = serde_json::from_str(json).unwrap();
  assert!(matches!(req, PlaybackRequest::LastPlayed));
}

#[test]
fn deserialize_recents_with_limit() {
  let json = r#"{"action": "recents", "limit": 10}"#;
  let req: PlaybackRequest = serde_json::from_str(json).unwrap();
  match req {
    PlaybackRequest::Recents { limit } => assert_eq!(limit, Some(10)),
    _ => panic!("expected Recents"),
  }
}

#[test]
fn deserialize_recents_without_limit() {
  let json = r#"{"action": "recents"}"#;
  let req: PlaybackRequest = serde_json::from_str(json).unwrap();
  match req {
    PlaybackRequest::Recents { limit } => assert_eq!(limit, None),
    _ => panic!("expected Recents"),
  }
}

#[test]
fn serialize_last_played_response() {
  let resp = PlaybackResponse::LastPlayed { track: None };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "last-played");
}

#[test]
fn serialize_recents_response() {
  let resp = PlaybackResponse::Recents { tracks: vec![] };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "recents");
}

#[test]
fn serialize_error_response() {
  let resp = PlaybackResponse::Error {
    message: "fail".into(),
  };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "error");
  assert_eq!(json["message"], "fail");
}

#[test]
fn track_skips_none_optional_fields() {
  let track = sample_track();
  let json = serde_json::to_value(&track).unwrap();
  assert!(!json.as_object().unwrap().contains_key("album_art"));
  assert!(!json.as_object().unwrap().contains_key("albumArt"));
  assert!(!json.as_object().unwrap().contains_key("url"));
  assert!(!json.as_object().unwrap().contains_key("preview_url"));
  assert!(!json.as_object().unwrap().contains_key("progress_ms"));
  assert!(!json.as_object().unwrap().contains_key("played_at"));
}
