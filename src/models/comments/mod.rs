//! # Comment Models
//!
//! Data models for blog comments, replies, and the WebSocket
//! protocol used for real-time comment operations.
//!
//! # Sub-modules
//!
//! | Module       | Contents                                  |
//! |--------------|-------------------------------------------|
//! | [`model`]    | `BlogComment`, `PopulatedComment`         |
//! | [`protocol`] | `CommentRequest`, `CommentResponse`, etc. |

mod model;
mod protocol;

pub use model::{BlogComment, PopulatedComment};
pub use protocol::{CommentEdit, CommentEvent, CommentRequest, CommentResponse};
