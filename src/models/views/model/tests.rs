//! Tests for PageViews model: construction, equality,
//! serde round-trips, and ByteString conversions.

use super::*;
use bytestring::ByteString;

#[test]
fn with_sets_count_to_zero() {
  let pv = PageViews::with("/blog/hello".into());
  assert_eq!(pv.route, "/blog/hello");
  assert_eq!(pv.count, 0);
}

#[test]
fn partial_eq_same_route_different_count() {
  let a = PageViews {
    route: "/home".into(),
    count: 5,
  };
  let b = PageViews {
    route: "/home".into(),
    count: 99,
  };
  assert_eq!(a, b);
}

#[test]
fn partial_eq_different_route() {
  let a = PageViews {
    route: "/home".into(),
    count: 0,
  };
  let b = PageViews {
    route: "/about".into(),
    count: 0,
  };
  assert_ne!(a, b);
}

#[test]
fn default_is_empty_route_zero_count() {
  let pv = PageViews::default();
  assert_eq!(pv.route, "");
  assert_eq!(pv.count, 0);
}

#[test]
fn serde_round_trip() {
  let original = PageViews {
    route: "/blog".into(),
    count: 42,
  };
  let json = serde_json::to_string(&original).unwrap();
  let restored: PageViews = serde_json::from_str(&json).unwrap();
  assert_eq!(restored.route, original.route);
  assert_eq!(restored.count, original.count);
}

#[test]
fn bytestring_conversion_round_trip() {
  let original = PageViews {
    route: "/test".into(),
    count: 7,
  };
  let bs: ByteString = original.clone().into();
  let restored: PageViews = bs.into();
  assert_eq!(restored.route, "/test");
  assert_eq!(restored.count, 7);
}

#[test]
fn from_bytestring_invalid_json_falls_back_to_default() {
  let bs = ByteString::from("not valid json");
  let pv: PageViews = bs.into();
  assert_eq!(pv, PageViews::default());
}

#[test]
fn from_bytestring_empty_string_falls_back_to_default() {
  let bs = ByteString::from("");
  let pv: PageViews = bs.into();
  assert_eq!(pv, PageViews::default());
}
