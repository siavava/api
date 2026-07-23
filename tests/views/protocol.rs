//! Tests for views request/response protocol
//! serialization and deserialization.

use server::models::views::{PageViews, ViewsRequest, ViewsResponse};

#[test]
fn deserialize_views_request_list() {
  let json = r#"{"action": "list"}"#;
  let req: ViewsRequest = serde_json::from_str(json).unwrap();
  assert!(matches!(req, ViewsRequest::List { namespace: None }));
}

#[test]
fn deserialize_views_request_get() {
  let json = r#"{"action": "get", "path": "/blog"}"#;
  let req: ViewsRequest = serde_json::from_str(json).unwrap();
  match req {
    ViewsRequest::Get { path } => {
      assert_eq!(path, "/blog")
    }
    _ => panic!("expected Get variant"),
  }
}

#[test]
fn serialize_views_response_list_has_type_tag() {
  let resp = ViewsResponse::List {
    views: vec![PageViews {
      route: "/home".into(),
      count: 3,
    }],
  };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "list");
  assert!(json["views"].is_array());
}

#[test]
fn serialize_views_response_update_flattens_fields() {
  let resp = ViewsResponse::Update {
    views: PageViews {
      route: "/blog".into(),
      count: 10,
    },
  };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "update");
  assert_eq!(json["route"], "/blog");
  assert_eq!(json["count"], 10);
}

#[test]
fn serialize_views_response_active_count_tag() {
  let resp = ViewsResponse::ActiveCount { count: 5 };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "active-count");
  assert_eq!(json["count"], 5);
}
