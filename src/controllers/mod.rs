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
//! | [`events`]   | Generic SSE broadcaster backed by MongoDB change streams.  |
//! | [`location`] | Location tracking — last known position and visit history. |
//! | [`views`]    | Page view counting and retrieval.                          |
//!
//! # Re-exports
//!
//! * [`EventsBroadcaster`] — the generic SSE broadcaster from [`events`].

/// Core logic for blog comments (CRUD, likes, replies).
pub mod comments;
/// Generic SSE broadcaster backed by MongoDB change streams.
pub mod events;
/// Location tracking (last known + history).
pub mod location;
/// Page view counting and retrieval.
pub mod views;

pub use events::EventsBroadcaster;
