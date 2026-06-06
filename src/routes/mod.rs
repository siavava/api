//! # Routes
//!
//! HTTP endpoint definitions and request handling.
//!
//! Each sub-module registers its endpoints under a path prefix via a
//! `register(cfg)` function. The top-level [`register`] function composes
//! them all.
//!
//! # Sub-modules
//!
//! | Module       | Prefix           | Protocol      | Description                                  |
//! |--------------|------------------|---------------|----------------------------------------------|
//! | [`connect`]  | `/api/connect/`  | WebSocket     | Unified real-time endpoint (comments + views).|
//! | [`comments`] | `/comments/`     | WebSocket     | Legacy comment-only WebSocket.               |
//! | [`location`] | `/location/`     | REST (GET)    | Location tracking read/update.               |
//! | [`quotes`]   | `/` + `/quotes/` | REST + HTML   | Quote display and retrieval.                 |
//! | [`views`]    | `/views/`        | REST + SSE    | Page view counts and live updates.           |

/// WebSocket endpoint for real-time comment operations.
pub mod comments;
/// Unified WebSocket endpoint for comments and view-count watch.
mod connect;
/// REST endpoint for location tracking.
mod location;
/// REST endpoint for OpenGraph metadata extraction.
mod opengraph;
/// Handlers for Spotify playback data (used by [`connect`]).
pub mod playback;
/// REST + HTML endpoints for quotes.
mod quotes;
/// REST auth + WebSocket endpoints for the study network. Isolated from blog.
mod study;
/// REST + SSE endpoints for page view counts.
mod views;

/// Registers all route modules under their respective path prefixes.
///
/// Called once during app setup in [`main`](crate::main).
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  quotes::register(cfg);
  views::register(cfg);
  location::register(cfg);
  comments::register(cfg);
  connect::register(cfg);
  opengraph::register(cfg);
  study::register(cfg);
}
