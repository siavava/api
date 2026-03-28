//! # OpenGraph Model
//!
//! Data model for OpenGraph metadata extracted from web pages.

use serde::{Deserialize, Serialize};

/// OpenGraph metadata extracted from a web page.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenGraphData {
  /// The page title (`og:title` or `<title>`).
  pub title: Option<String>,
  /// The page description (`og:description` or `<meta name="description">`).
  pub description: Option<String>,
  /// The preview image URL (`og:image`).
  pub image: Option<String>,
  /// The site name (`og:site_name`).
  pub site_name: Option<String>,
  /// The canonical URL (`og:url` or the requested URL).
  pub url: String,
  /// The favicon URL.
  pub favicon: Option<String>,
  /// The hostname extracted from the URL.
  pub hostname: Option<String>,
}

#[cfg(test)]
mod tests;
