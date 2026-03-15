//! # WebSocket Utilities
//!
//! Shared helpers for WebSocket session management used by route handlers.

use actix_ws::Session;
use serde::Serialize;
use tracing::error;

/// Serializes a value as JSON and sends it over a WebSocket session.
///
/// # Returns
///
/// * `true` — the message was sent successfully.
/// * `false` — the send failed (the caller should close the connection).
///
/// Serialization errors are logged but do **not** return `false`, since
/// they indicate a bug in the response type rather than a broken
/// connection.
pub async fn send_json<T: Serialize>(session: &mut Session, value: &T) -> bool {
  match serde_json::to_string(value) {
    Ok(json) => {
      if let Err(e) = session.text(json).await {
        error!("failed to send ws message: {e}");
        return false;
      }
      true
    }
    Err(e) => {
      error!("failed to serialize response: {e}");
      true
    }
  }
}
