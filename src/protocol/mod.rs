//! # Protocol
//!
//! Generic, reusable infrastructure for real-time communication protocols.
//!
//! These modules are protocol-level utilities — they know nothing about
//! domain concepts like comments, views, or locations. Domain-specific
//! handlers live in [`crate::routes`] and [`crate::controllers`].
//!
//! | Module | Protocol | Description                                            |
//! |--------|----------|--------------------------------------------------------|
//! | [`socket`] | WebSocket| JSON serialization helpers for WebSocket sessions. |
//! | [`sse`]| SSE      | Generic broadcaster backed by MongoDB change streams.  |

/// WebSocket session utilities.
pub mod socket;
/// Server-Sent Events broadcaster backed by MongoDB change streams.
pub mod sse;
