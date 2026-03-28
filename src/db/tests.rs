//! Tests for database helpers: ObjectId parsing and DB name.

use super::*;

#[test]
fn db_name_is_feed_dev_in_debug() {
  assert_eq!(DB_NAME, "feed-dev");
}

#[test]
fn parse_oid_valid_hex() {
  let hex = "507f1f77bcf86cd799439011";
  let result = parse_oid(hex);
  assert!(result.is_ok());
  assert_eq!(result.unwrap().to_hex(), hex);
}

#[test]
fn parse_oid_invalid_string() {
  let result = parse_oid("not-a-valid-oid");
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("invalid id"));
}

#[test]
fn parse_oid_empty_string() {
  let result = parse_oid("");
  assert!(result.is_err());
}

#[test]
fn parse_oids_mixed_valid_and_invalid() {
  let ids = vec![
    "507f1f77bcf86cd799439011".to_string(),
    "not-valid".to_string(),
    "507f191e810c19729de860ea".to_string(),
  ];
  let result = parse_oids(&ids);
  assert_eq!(result.len(), 2);
  assert_eq!(result[0].to_hex(), "507f1f77bcf86cd799439011");
  assert_eq!(result[1].to_hex(), "507f191e810c19729de860ea");
}

#[test]
fn parse_oids_empty_slice() {
  let ids: Vec<String> = vec![];
  let result = parse_oids(&ids);
  assert!(result.is_empty());
}

#[test]
fn parse_oids_all_invalid() {
  let ids = vec!["bad".to_string(), "also-bad".to_string(), "".to_string()];
  let result = parse_oids(&ids);
  assert!(result.is_empty());
}
