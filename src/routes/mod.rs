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
//! | Module       | Prefix       | Protocol      | Description                          |
//! |--------------|--------------|---------------|--------------------------------------|
//! | [`comments`] | `/comments/` | WebSocket     | Real-time comment operations.        |
//! | [`location`] | `/location/` | REST (GET)    | Location tracking read/update.       |
//! | [`quotes`]   | `/` + `/quotes/` | REST + HTML | Quote display and retrieval.     |
//! | [`views`]    | `/views/`    | REST + SSE    | Page view counts and live updates.   |

/// WebSocket endpoint for real-time comment operations.
mod comments;
/// REST endpoint for location tracking.
mod location;
/// REST + HTML endpoints for quotes.
mod quotes;
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
}
