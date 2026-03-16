//! # Health Check
//!
//! Diagnostic response model for the health-check scope on `/api/connect/`.

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::{LazyLock, atomic::Ordering};
use std::time::Instant;

use crate::AppState;

/// Process start time, initialized on first access.
static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

/// Diagnostic snapshot returned by the health-check endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct HealthDiagnostics {
  /// Process uptime in seconds.
  pub uptime_secs: f64,
  /// Current server time (UTC, ISO-8601).
  pub server_time: DateTime<Utc>,
  /// Number of currently connected WebSocket clients.
  pub active_clients: usize,
  /// Whether the MongoDB connection is healthy.
  pub db_connected: bool,
}

impl HealthDiagnostics {
  /// Collects a diagnostics snapshot from the current app state.
  pub async fn collect(state: &AppState) -> Self {
    let uptime = START_TIME.elapsed();
    let db_connected = state
      .db_client
      .database("admin")
      .run_command(mongodb::bson::doc! { "ping": 1 })
      .await
      .is_ok();

    Self {
      uptime_secs: uptime.as_secs_f64(),
      server_time: Utc::now(),
      active_clients: state.active_clients.load(Ordering::Relaxed),
      db_connected,
    }
  }
}
