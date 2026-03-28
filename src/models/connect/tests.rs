//! Tests for the unified WebSocket connect protocol:
//! request parsing, scope routing, and response
//! serialization.

use super::*;
use serde_json;

// ── ConnectRequest::parse ──────────────────────────

#[test]
fn parse_no_scope_defaults_to_comments() {
  let input = r#"{"action": "list", "path": "/blog/post-1"}"#;
  let req = ConnectRequest::parse(input).unwrap();
  assert!(matches!(req, ConnectRequest::Comments(_)));
}

#[test]
fn parse_explicit_comments_scope_list() {
  let input = r#"{
    "scope": "comments",
    "action": "list",
    "path": "/blog/post-1"
  }"#;
  let req = ConnectRequest::parse(input).unwrap();
  assert!(matches!(req, ConnectRequest::Comments(_)));
}

#[test]
fn parse_views_scope_list() {
  let input = r#"{"scope": "views", "action": "list"}"#;
  let req = ConnectRequest::parse(input).unwrap();
  assert!(matches!(req, ConnectRequest::Views(_)));
}

#[test]
fn parse_views_scope_get() {
  let input = r#"{
    "scope": "views",
    "action": "get",
    "path": "/blog/post-1"
  }"#;
  let req = ConnectRequest::parse(input).unwrap();
  assert!(matches!(req, ConnectRequest::Views(_)));
}

#[test]
fn parse_health_scope_defaults() {
  let input = r#"{"scope": "health"}"#;
  let req = ConnectRequest::parse(input).unwrap();
  match req {
    ConnectRequest::Health(opts) => {
      assert!(!opts.quotes)
    }
    other => {
      panic!("expected Health, got {:?}", other)
    }
  }
}

#[test]
fn parse_health_scope_with_quotes() {
  let input = r#"{"scope": "health", "quotes": true}"#;
  let req = ConnectRequest::parse(input).unwrap();
  match req {
    ConnectRequest::Health(opts) => {
      assert!(opts.quotes)
    }
    other => {
      panic!("expected Health, got {:?}", other)
    }
  }
}

#[test]
fn parse_opengraph_scope() {
  let input = r#"{
    "scope": "opengraph",
    "url": "https://example.com"
  }"#;
  let req = ConnectRequest::parse(input).unwrap();
  match req {
    ConnectRequest::OpenGraph(og) => {
      assert_eq!(og.url, "https://example.com")
    }
    other => {
      panic!("expected OpenGraph, got {:?}", other)
    }
  }
}

#[test]
fn parse_playback_scope_last_played() {
  let input = r#"{
    "scope": "playback",
    "action": "last-played"
  }"#;
  let req = ConnectRequest::parse(input).unwrap();
  assert!(matches!(req, ConnectRequest::Playback(_)));
}

#[test]
fn parse_invalid_json() {
  let err = ConnectRequest::parse("not json").unwrap_err();
  assert!(
    err.contains("invalid JSON"),
    "expected 'invalid JSON' in error, \
     got: {err}"
  );
}

#[test]
fn parse_invalid_scope_value() {
  let input = r#"{"scope": "unknown"}"#;
  let err = ConnectRequest::parse(input).unwrap_err();
  assert!(
    err.contains("invalid scope"),
    "expected 'invalid scope' in error, \
     got: {err}"
  );
}

#[test]
fn parse_valid_scope_invalid_inner_payload() {
  // opengraph requires a `url` field
  let input = r#"{"scope": "opengraph"}"#;
  let err = ConnectRequest::parse(input).unwrap_err();
  assert!(
    err.contains("invalid opengraph request"),
    "expected 'invalid opengraph request' \
     in error, got: {err}"
  );
}

// ── ConnectResponse serialization ──────────────────

#[test]
fn connect_response_comments_has_scope_tag() {
  let resp = ConnectResponse::Comments(CommentResponse::Error {
    message: "test".into(),
  });
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["scope"], "comments");
}

#[test]
fn connect_response_views_has_scope_tag() {
  let resp = ConnectResponse::Views(ViewsResponse::List { views: vec![] });
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["scope"], "views");
}

#[test]
fn connect_response_health_has_scope_tag() {
  use chrono::Utc;
  let resp = ConnectResponse::Health(HealthDiagnostics {
    uptime_secs: 1.0,
    server_time: Utc::now(),
    active_clients: 0,
    db_connected: true,
    quotes: None,
  });
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["scope"], "health");
}

#[test]
fn connect_response_playback_has_scope_tag() {
  let resp =
    ConnectResponse::Playback(PlaybackResponse::LastPlayed { track: None });
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["scope"], "playback");
}

#[test]
fn connect_response_opengraph_has_scope_tag() {
  let resp = ConnectResponse::OpenGraph(OpenGraphResponse::Error {
    message: "fail".into(),
  });
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["scope"], "opengraph");
}

// ── OpenGraphResponse serialization ────────────────

#[test]
fn opengraph_response_data_has_action_tag() {
  let resp = OpenGraphResponse::Data(OpenGraphData {
    title: Some("Test".into()),
    description: None,
    image: None,
    site_name: None,
    url: "https://example.com".into(),
    favicon: None,
    hostname: None,
  });
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["action"], "data");
}

#[test]
fn opengraph_response_error_has_action_tag() {
  let resp = OpenGraphResponse::Error {
    message: "not found".into(),
  };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["action"], "error");
  assert_eq!(json["message"], "not found");
}
