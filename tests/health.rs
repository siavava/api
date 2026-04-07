//! Tests for health check model defaults and
//! deserialization.

use server::models::health::HealthOptions;

#[test]
fn health_options_default() {
  let opts = HealthOptions::default();
  assert!(!opts.quotes);
}

#[test]
fn health_options_with_quotes_true() {
  let json = r#"{"quotes": true}"#;
  let opts: HealthOptions = serde_json::from_str(json).unwrap();
  assert!(opts.quotes);
}

#[test]
fn health_options_empty_json_uses_defaults() {
  let json = r#"{}"#;
  let opts: HealthOptions = serde_json::from_str(json).unwrap();
  assert!(!opts.quotes);
}
