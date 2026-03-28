//! Tests for quote model serialization and the
//! built-in quotes collection.

use super::*;

#[test]
fn get_all_returns_non_empty() {
  let quotes = get_all();
  assert!(!quotes.is_empty());
}

#[test]
fn each_quote_has_non_empty_fields() {
  for quote in get_all() {
    assert!(!quote.text.is_empty(), "quote text should not be empty");
    assert!(!quote.author.is_empty(), "quote author should not be empty");
  }
}

#[test]
fn quote_serde_round_trip() {
  let quote = Quote {
    text: "To be or not to be.".into(),
    author: "Shakespeare".into(),
  };
  let json = serde_json::to_string(&quote).unwrap();
  let deserialized: Quote = serde_json::from_str(&json).unwrap();
  assert_eq!(deserialized.text, "To be or not to be.");
  assert_eq!(deserialized.author, "Shakespeare");
}
