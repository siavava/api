//! Tests for OpenGraph metadata extraction:
//! meta tag content parsing and favicon resolution.

use super::*;
use scraper::{Html, Selector};
use url::Url;

#[test]
fn meta_content_extracts_content_attribute() {
  let html = Html::parse_document(
    r#"<html><head>
      <meta property="og:title" content="Hello">
    </head></html>"#,
  );
  let selector = Selector::parse(r#"meta[property="og:title"]"#).unwrap();
  assert_eq!(meta_content(&html, &selector), Some("Hello".to_string()));
}

#[test]
fn meta_content_returns_none_when_no_match() {
  let html = Html::parse_document("<html><head></head></html>");
  let selector = Selector::parse(r#"meta[property="og:title"]"#).unwrap();
  assert_eq!(meta_content(&html, &selector), None);
}

#[test]
fn meta_content_returns_none_when_no_content_attr() {
  let html = Html::parse_document(
    r#"<html><head>
      <meta property="og:title">
    </head></html>"#,
  );
  let selector = Selector::parse(r#"meta[property="og:title"]"#).unwrap();
  assert_eq!(meta_content(&html, &selector), None);
}

#[test]
fn resolve_favicon_absolute_url_returned_as_is() {
  let html = Html::parse_document(
    r#"<html><head>
      <link rel="icon"
        href="https://example.com/icon.png">
    </head></html>"#,
  );
  let base = Url::parse("https://example.com").unwrap();
  assert_eq!(
    resolve_favicon(&html, &base),
    Some("https://example.com/icon.png".to_string()),
  );
}

#[test]
fn resolve_favicon_relative_url_resolved_against_base() {
  let html = Html::parse_document(
    r#"<html><head>
      <link rel="icon"
        href="/static/icon.png">
    </head></html>"#,
  );
  let base = Url::parse("https://example.com").unwrap();
  assert_eq!(
    resolve_favicon(&html, &base),
    Some("https://example.com/static/icon.png".to_string()),
  );
}

#[test]
fn resolve_favicon_falls_back_to_favicon_ico() {
  let html = Html::parse_document("<html><head></head></html>");
  let base = Url::parse("https://example.com").unwrap();
  assert_eq!(
    resolve_favicon(&html, &base),
    Some("https://example.com/favicon.ico".to_string()),
  );
}
