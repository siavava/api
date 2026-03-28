//! # Views Handlers
//!
//! Per-protocol handlers for view-count endpoints.
//!
//! | Module | Protocol | Description                          |
//! |--------|----------|--------------------------------------|
//! | [`rest`] | REST   | GET/POST/DELETE for view counts.      |
//! | [`sse`]  | SSE    | Real-time view-count stream.          |
//! | [`socket`] | WS   | WebSocket requests via `/api/connect/`.|

/// REST handlers (GET, POST, DELETE).
pub mod rest;
/// WebSocket handler for the unified `/api/connect/` endpoint.
pub mod socket;
/// SSE handlers (watch stream + test page).
pub mod sse;
