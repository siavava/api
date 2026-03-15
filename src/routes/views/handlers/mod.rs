//! # Views Handlers
//!
//! Per-protocol handlers for view-count endpoints.
//!
//! | Module | Protocol | Description                          |
//! |--------|----------|--------------------------------------|
//! | [`rest`] | REST   | GET/POST/DELETE for view counts.      |
//! | [`sse`]  | SSE    | Real-time view-count stream.          |
//! | [`ws`]   | WS     | WebSocket requests via `/api/connect/`.|

/// REST handlers (GET, POST, DELETE).
pub mod rest;
/// SSE handlers (watch stream + test page).
pub mod sse;
/// WebSocket handler for the unified connect endpoint.
pub mod ws;
