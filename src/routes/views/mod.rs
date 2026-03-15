//! # Views Route
//!
//! REST, SSE, and WebSocket handlers for page view counts.
//!
//! # Endpoints
//!
//! | Method | Path                | Protocol | Description                                    |
//! |--------|---------------------|----------|------------------------------------------------|
//! | GET    | `/views/`           | REST     | Read view counts (single or all).              |
//! | POST   | `/views/`           | REST     | Upsert one or more view-count records.         |
//! | DELETE | `/views/`           | REST     | Delete a route's view-count document.          |
//! | GET    | `/views/watch/`     | SSE      | Stream of real-time view-count updates.        |
//! | GET    | `/views/watch/test/`| HTML     | Static HTML page for testing the SSE endpoint. |

pub mod handlers;

use actix_web::web::scope;

/// Registers all `/views/` endpoints (REST, SSE).
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(
    scope("/views")
      .service(handlers::rest::get_views)
      .service(handlers::rest::delete_views)
      .service(handlers::rest::insert_views)
      .service(handlers::sse::watch_views)
      .service(handlers::sse::watch_views_test),
  );
}
