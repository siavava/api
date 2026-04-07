//! # Health Check
//!
//! Diagnostic response model for the health-check scope on `/api/connect/`.

use super::quotes::Quote;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
  sync::{LazyLock, atomic::Ordering},
  time::{Duration, Instant},
};

use crate::AppState;

/// Process start time, initialized on first access.
static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

/// Options parsed from the health-check request payload.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct HealthOptions {
  /// If `true`, include all quotes in the response.
  #[serde(default)]
  pub quotes: bool,
}

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
  /// All quotes (only present when requested with `quotes: true`).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quotes: Option<&'static [Quote]>,
}

impl HealthDiagnostics {
  /// Collects a diagnostics snapshot from the current app state.
  pub async fn collect(state: &AppState, options: &HealthOptions) -> Self {
    let uptime = START_TIME.elapsed();
    let db_connected = tokio::time::timeout(
      Duration::from_secs(5),
      state
        .db_client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 }),
    )
    .await
    .map(|r| r.is_ok())
    .unwrap_or(false);

    let quotes = if options.quotes {
      Some(super::quotes::get_all())
    } else {
      None
    };

    Self {
      uptime_secs: uptime.as_secs_f64(),
      server_time: Utc::now(),
      active_clients: state.active_clients.load(Ordering::Relaxed),
      db_connected,
      quotes,
    }
  }
}
