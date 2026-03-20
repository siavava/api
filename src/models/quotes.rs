//! # Quotes
//!
//! Data model for quotes, shared between the REST endpoint and health-check.

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// A single quote with its text and attribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
  /// The quote body (may contain HTML markup).
  pub text: String,
  /// Who said or wrote the quote.
  pub author: String,
}

/// Wrapper for deserializing the static `quotes.json` file.
#[derive(Debug, Deserialize)]
pub struct QuoteData {
  /// List of all quotes loaded from the JSON file.
  pub quotes: Vec<Quote>,
}

/// Parsed quotes, deserialized once on first access.
static QUOTES: LazyLock<Vec<Quote>> = LazyLock::new(|| {
  let data: QuoteData = serde_json::from_str(include_str!("../static/quotes.json"))
    .expect("quotes.json was not well-formatted");
  data.quotes
});

/// Returns a reference to all quotes (parsed once, cached forever).
pub fn get_all() -> &'static [Quote] {
  &QUOTES
}
