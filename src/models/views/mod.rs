//! # Views Models
//!
//! Data models for page view counts and the WebSocket protocol
//! used for real-time view-count updates.
//!
//! # Sub-modules
//!
//! | Module       | Contents                          |
//! |--------------|-----------------------------------|
//! | [`model`]    | `PageViews`                       |
//! | [`protocol`] | `ViewsResponse`, `ViewEvent`      |

mod model;
mod protocol;

pub use model::PageViews;
pub use protocol::{ViewEvent, ViewsRequest, ViewsResponse};
