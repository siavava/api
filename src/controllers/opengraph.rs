//! # OpenGraph Controller
//!
//! Fetches a web page and extracts OpenGraph metadata from its HTML.

use crate::models::opengraph::OpenGraphData;
use scraper::{Html, Selector};
use std::sync::LazyLock;
use std::time::Duration;
use url::Url;

/// Shared HTTP client with a 10-second timeout.
static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
  reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .build()
    .expect("failed to build HTTP client")
});

/// Fetches the given URL and parses OpenGraph metadata from the HTML.
pub async fn fetch_opengraph(target_url: &str) -> Result<OpenGraphData, String> {
  let parsed = Url::parse(target_url).map_err(|e| format!("Invalid URL: {e}"))?;
  let hostname = parsed.host_str().map(String::from);

  let response = HTTP_CLIENT
    .get(target_url)
    .send()
    .await
    .map_err(|e| format!("Failed to fetch URL: {e}"))?;

  let final_url = response.url().to_string();

  let html_text = response
    .text()
    .await
    .map_err(|e| format!("Failed to read response body: {e}"))?;

  let document = Html::parse_document(&html_text);

  let og_title = meta_property(&document, "og:title");
  let og_description = meta_property(&document, "og:description");
  let og_image = meta_property(&document, "og:image");
  let og_site_name = meta_property(&document, "og:site_name");
  let og_url = meta_property(&document, "og:url");

  // Fallbacks for title and description
  let title = og_title.or_else(|| {
    Selector::parse("title")
      .ok()
      .and_then(|sel| document.select(&sel).next())
      .map(|el| el.text().collect::<String>())
  });

  let description = og_description.or_else(|| meta_name(&document, "description"));

  // Resolve favicon
  let favicon = resolve_favicon(&document, &parsed);

  Ok(OpenGraphData {
    title,
    description,
    image: og_image,
    site_name: og_site_name,
    url: og_url.unwrap_or(final_url),
    favicon,
    hostname,
  })
}

/// Extracts a `<meta property="..." content="...">` value.
fn meta_property(document: &Html, property: &str) -> Option<String> {
  let selector =
    Selector::parse(&format!("meta[property=\"{property}\"]")).ok()?;
  document
    .select(&selector)
    .next()
    .and_then(|el| el.value().attr("content"))
    .map(String::from)
}

/// Extracts a `<meta name="..." content="...">` value.
fn meta_name(document: &Html, name: &str) -> Option<String> {
  let selector =
    Selector::parse(&format!("meta[name=\"{name}\"]")).ok()?;
  document
    .select(&selector)
    .next()
    .and_then(|el| el.value().attr("content"))
    .map(String::from)
}

/// Resolves the favicon URL from `<link rel="icon">` tags, falling back to `/favicon.ico`.
fn resolve_favicon(document: &Html, base_url: &Url) -> Option<String> {
  let icon = Selector::parse("link[rel=\"icon\"], link[rel=\"shortcut icon\"]")
    .ok()
    .and_then(|sel| {
      document
        .select(&sel)
        .next()
        .and_then(|el| el.value().attr("href"))
        .map(String::from)
    });

  match icon {
    Some(href) if href.starts_with("http") => Some(href),
    Some(href) => base_url.join(&href).ok().map(|u| u.to_string()),
    None => {
      // Default to /favicon.ico
      base_url.join("/favicon.ico").ok().map(|u| u.to_string())
    }
  }
}
