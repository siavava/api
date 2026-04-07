//! Tests for OpenGraph model serialization with
//! camelCase field renaming.

use server::models::opengraph::OpenGraphData;

fn sample() -> OpenGraphData {
  OpenGraphData {
    title: Some("Title".into()),
    description: Some("Desc".into()),
    image: Some("https://img.png".into()),
    site_name: Some("Site".into()),
    url: "https://example.com".into(),
    favicon: Some("https://example.com/fav.ico".into()),
    hostname: Some("example.com".into()),
  }
}

#[test]
fn serde_round_trip_camel_case() {
  let data = sample();
  let json = serde_json::to_string(&data).unwrap();
  assert!(json.contains("siteName"));
  assert!(!json.contains("site_name")); // verify camelCase keys
  let deserialized: OpenGraphData = serde_json::from_str(&json).unwrap();
  assert_eq!(deserialized.title.as_deref(), Some("Title"));
  assert_eq!(deserialized.site_name.as_deref(), Some("Site"));
}

#[test]
fn optional_fields_serialize_as_null() {
  let data = OpenGraphData {
    title: None,
    description: None,
    image: None,
    site_name: None,
    url: "https://example.com".into(),
    favicon: None,
    hostname: None,
  };
  let json = serde_json::to_value(&data).unwrap();
  assert!(json["title"].is_null());
  assert!(json["description"].is_null());
  assert!(json["siteName"].is_null());
}

#[test]
fn deserialize_from_camel_case_json() {
  let json = r#"{
    "title": "Test",
    "description": null,
    "image": null,
    "siteName": "MySite",
    "url": "https://example.com",
    "favicon": null,
    "hostname": "example.com"
  }"#;
  let data: OpenGraphData = serde_json::from_str(json).unwrap();
  assert_eq!(data.title.as_deref(), Some("Test"));
  assert_eq!(data.site_name.as_deref(), Some("MySite"));
  assert_eq!(data.url, "https://example.com");
  assert_eq!(data.hostname.as_deref(), Some("example.com"));
}
