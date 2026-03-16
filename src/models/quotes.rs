//! # Quotes
//!
//! Data model for quotes, shared between the REST endpoint and health-check.

use serde::{Deserialize, Serialize};

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

/// Loads all quotes from the embedded static JSON file.
pub fn get_all() -> Vec<Quote> {
  let data: QuoteData = serde_json::from_str(include_str!("../static/quotes.json"))
    .expect("quotes.json was not well-formatted");
  data.quotes
}
