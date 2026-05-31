//! # Models
//!
//! Data models (structs and enums) shared across the API.
//!
//! Each sub-module defines the types for a specific feature area. These types
//! are used by both [`controllers`](crate::controllers) (for DB operations) and
//! [`routes`](crate::routes) (for request/response serialization).
//!
//! # Sub-modules
//!
//! | Module       | Description                                                        |
//! |--------------|--------------------------------------------------------------------|
//! | [`comments`] | Blog comment structs, WebSocket protocol messages, and reply trees. |
//! | [`location`] | Location tracking data (city + state).                             |
//! | [`views`]    | Page view count tracking.                                          |

/// Data models for blog comments, replies, and WebSocket protocol messages.
pub mod comments;
/// Unified WebSocket protocol types for `/api/connect`.
pub mod connect;
/// Health-check diagnostics model.
pub mod health;
/// Data model for location tracking.
pub mod location;
/// Data models for owner-published "now" status slots.
pub mod now;
/// Data model for OpenGraph metadata.
pub mod opengraph;
/// Data models for Spotify playback.
pub mod playback;
/// Data model for quotes.
pub mod quotes;
/// Data model for page view counts.
pub mod views;
