//! Tests for location model serialization and
//! ByteString conversion.

use super::*;

#[test]
fn constructor() {
  let loc = LocationData::with("San Francisco".into(), "California".into());
  assert_eq!(loc.city, "San Francisco");
  assert_eq!(loc.state, "California");
}

#[test]
fn serde_round_trip() {
  let loc = LocationData::with("Denver".into(), "Colorado".into());
  let json = serde_json::to_string(&loc).unwrap();
  let deserialized: LocationData = serde_json::from_str(&json).unwrap();
  assert_eq!(loc, deserialized);
}

#[test]
fn bytestring_round_trip() {
  let loc = LocationData::with("Austin".into(), "Texas".into());
  let bytes: ByteString = loc.clone().into();
  let restored: LocationData = bytes.into();
  assert_eq!(loc, restored);
}

#[test]
fn default_is_empty() {
  let loc = LocationData::default();
  assert_eq!(loc.city, "");
  assert_eq!(loc.state, "");
}

#[test]
fn partial_eq() {
  let a = LocationData::with("A".into(), "B".into());
  let b = LocationData::with("A".into(), "B".into());
  let c = LocationData::with("X".into(), "Y".into());
  assert_eq!(a, b);
  assert_ne!(a, c);
}

#[test]
fn invalid_bytestring_yields_default() {
  let bytes = ByteString::from("not valid json");
  let loc: LocationData = bytes.into();
  assert_eq!(loc, LocationData::default());
}
