//! Tests for views request/response protocol
//! serialization and deserialization.

use server::models::{
  connect::ConnectRequest,
  views::{PageViews, ViewerLocation, ViewsRequest, ViewsResponse},
};

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
    location: None,
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

#[test]
fn serialize_views_response_update_with_location() {
  let resp = ViewsResponse::Update {
    views: PageViews {
      route: "<p>:/code".into(),
      count: 3,
    },
    location: Some(ViewerLocation {
      city: "Menlo Park".into(),
      state: "California".into(),
      lat: Some(37.45),
      lon: Some(-122.18),
    }),
  };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["location"]["city"], "Menlo Park");
  assert_eq!(json["location"]["state"], "California");
}

#[test]
fn deserialize_watch_request_with_location() {
  let json = r#"{"scope": "watch", "path": "<p>:/code",
    "city": "Menlo Park", "state": "California",
    "lat": 37.45, "lon": -122.18}"#;
  let req = ConnectRequest::parse(json).unwrap();
  match req {
    ConnectRequest::Watch(watch) => {
      assert_eq!(watch.path, "<p>:/code");
      assert_eq!(watch.city.as_deref(), Some("Menlo Park"));
      assert_eq!(watch.state.as_deref(), Some("California"));
      assert_eq!(watch.lat, Some(37.45));
    }
    other => panic!("expected watch request, got {other:?}"),
  }
}

#[test]
fn deserialize_watch_request_without_location_still_works() {
  let json = r#"{"scope": "watch", "path": "/blog/post-1"}"#;
  let req = ConnectRequest::parse(json).unwrap();
  match req {
    ConnectRequest::Watch(watch) => {
      assert_eq!(watch.path, "/blog/post-1");
      assert!(watch.city.is_none());
    }
    other => panic!("expected watch request, got {other:?}"),
  }
}
