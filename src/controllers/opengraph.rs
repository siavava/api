//! # OpenGraph Controller
//!
//! Fetches a web page and extracts OpenGraph metadata from its HTML.

use crate::models::opengraph::OpenGraphData;
use scraper::{Html, Selector};
use std::{sync::LazyLock, time::Duration};
use url::Url;

/// Shared HTTP client with a 10-second timeout.
static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
  reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .build()
    .expect("failed to build HTTP client")
});

/// Pre-parsed CSS selectors for OpenGraph metadata extraction.
struct Selectors {
  og_title: Selector,
  og_description: Selector,
  og_image: Selector,
  og_site_name: Selector,
  og_url: Selector,
  title: Selector,
  meta_description: Selector,
  favicon: Selector,
}

static SELECTORS: LazyLock<Selectors> = LazyLock::new(|| {
  let s = |css: &str| Selector::parse(css).expect("invalid selector");
  Selectors {
    og_title: s("meta[property=\"og:title\"]"),
    og_description: s("meta[property=\"og:description\"]"),
    og_image: s("meta[property=\"og:image\"]"),
    og_site_name: s("meta[property=\"og:site_name\"]"),
    og_url: s("meta[property=\"og:url\"]"),
    title: s("title"),
    meta_description: s("meta[name=\"description\"]"),
    favicon: s("link[rel=\"icon\"], link[rel=\"shortcut icon\"]"),
  }
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

  let s = &*SELECTORS;

  let og_title = meta_content(&document, &s.og_title);
  let og_description = meta_content(&document, &s.og_description);
  let og_image = meta_content(&document, &s.og_image);
  let og_site_name = meta_content(&document, &s.og_site_name);
  let og_url = meta_content(&document, &s.og_url);

  // Fallbacks for title and description
  let title = og_title.or_else(|| {
    document
      .select(&s.title)
      .next()
      .map(|el| el.text().collect::<String>())
  });

  let description = og_description.or_else(|| meta_content(&document, &s.meta_description));

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

/// Extracts the `content` attribute from the first element matching a selector.
fn meta_content(document: &Html, selector: &Selector) -> Option<String> {
  document
    .select(selector)
    .next()
    .and_then(|el| el.value().attr("content"))
    .map(String::from)
}

/// Resolves the favicon URL from `<link rel="icon">` tags, falling back to `/favicon.ico`.
fn resolve_favicon(document: &Html, base_url: &Url) -> Option<String> {
  let icon = document
    .select(&SELECTORS.favicon)
    .next()
    .and_then(|el| el.value().attr("href"))
    .map(String::from);

  match icon {
    Some(href) if href.starts_with("http") => Some(href),
    Some(href) => base_url.join(&href).ok().map(|u| u.to_string()),
    None => {
      // Default to /favicon.ico
      base_url.join("/favicon.ico").ok().map(|u| u.to_string())
    }
  }
}
