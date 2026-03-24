//! # Controllers
//!
//! Core logic layer for the API.
//!
//! Each sub-module encapsulates the domain logic for a specific feature area,
//! operating directly against MongoDB collections and returning domain types
//! from [`crate::models`].
//!
//! # Sub-modules
//!
//! | Module       | Description                                                |
//! |--------------|------------------------------------------------------------|
//! | [`comments`] | CRUD, likes, and recursive reply-tree management for blog comments.  |
//! | [`location`] | Location tracking — last known position and visit history. |
//! | [`views`]    | Page view counting and retrieval.                          |
//!
//! Generic protocol utilities (WebSocket helpers, SSE broadcaster) live in
//! [`crate::protocol`].

/// Core logic for blog comments (CRUD, likes, replies).
pub mod comments;
/// Location tracking (last known + history).
pub mod location;
/// OpenGraph metadata fetching and parsing.
pub mod opengraph;
/// Spotify playback data.
pub mod playback;
/// Page view counting and retrieval.
pub mod views;
