//! # Study Network Models
//!
//! Data models for the study site: authenticated users, notes, inline
//! annotations (text-anchored highlights), and per-section reading progress.
//!
//! These types back the `/study/*` REST + WebSocket surface and are entirely
//! separate from the blog scopes — the blog never touches them.

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

/// Serializes an `Option<ObjectId>` as a hex string (or `null`).
fn ser_oid<S: Serializer>(
  id: &Option<ObjectId>,
  s: S,
) -> Result<S::Ok, S::Error> {
  match id {
    Some(oid) => s.serialize_str(&oid.to_hex()),
    None => s.serialize_none(),
  }
}

// --------------------------------------------------------------------------- //
// Users & auth
// --------------------------------------------------------------------------- //

/// Database model for a registered study user, stored in `study_users`.
///
/// The `password_hash` is serialized to MongoDB but must never be sent to a
/// client — use [`PublicUser`] for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  #[serde(
    alias = "_id",
    skip_serializing_if = "Option::is_none",
    serialize_with = "ser_oid"
  )]
  pub id: Option<ObjectId>,
  pub username: String,
  pub email: String,
  pub password_hash: String,
  #[serde(default)]
  pub created_time: String,
}

/// Client-facing user (no password hash).
#[derive(Debug, Clone, Serialize)]
pub struct PublicUser {
  pub id: String,
  pub username: String,
  pub email: String,
}

impl From<&User> for PublicUser {
  fn from(u: &User) -> Self {
    Self {
      id: u.id.map(|o| o.to_hex()).unwrap_or_default(),
      username: u.username.clone(),
      email: u.email.clone(),
    }
  }
}

/// JWT claims for an authenticated study session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
  /// Subject — the user's hex ObjectId.
  pub sub: String,
  pub username: String,
  /// Expiry (unix seconds).
  pub exp: usize,
}

/// `POST /study/auth/signup` request body.
#[derive(Debug, Deserialize)]
pub struct SignupRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

/// `POST /study/auth/login` request body.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
  /// Username or email.
  pub identifier: String,
  pub password: String,
}

/// Auth success response (returned by signup + login).
#[derive(Debug, Serialize)]
pub struct AuthResponse {
  pub token: String,
  pub user: PublicUser,
}

// --------------------------------------------------------------------------- //
// Notes
// --------------------------------------------------------------------------- //

/// A free-form note, optionally scoped to a book section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
  #[serde(
    alias = "_id",
    skip_serializing_if = "Option::is_none",
    serialize_with = "ser_oid"
  )]
  pub id: Option<ObjectId>,
  /// Owning user's hex id. Set server-side; never trusted from the client.
  #[serde(default)]
  pub user_id: String,
  /// Book key (`clrs` | `skiena` | `erickson`), if section-scoped.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub book_key: Option<String>,
  /// Route path of the section this note attaches to, if any.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub section_path: Option<String>,
  /// Human-readable label of the section (for the central summary view).
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub section_title: Option<String>,
  /// Chapter title of the source section.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub chapter: Option<String>,
  /// Section number (e.g. "2.1"), where the source text uses one.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub section_number: Option<String>,
  /// Page range `[start, end]` in the source PDF.
  #[serde(default)]
  pub pages: Vec<i64>,
  /// Formatted citation, e.g. "CLRS §2.1, pp. 17–21".
  #[serde(default)]
  pub citation: String,
  /// Topic tags for grouping/filtering in the summary & review views.
  #[serde(default)]
  pub topics: Vec<String>,
  #[serde(default)]
  pub title: String,
  /// Markdown body (may contain `$…$` / `$$…$$` LaTeX math).
  #[serde(default)]
  pub body: String,
  #[serde(default)]
  pub tags: Vec<String>,
  #[serde(default)]
  pub created_time: String,
  #[serde(default)]
  pub updated_time: String,
}

/// Client-supplied note fields (for create/update).
#[derive(Debug, Deserialize)]
pub struct NoteInput {
  pub id: Option<String>,
  pub book_key: Option<String>,
  pub section_path: Option<String>,
  pub section_title: Option<String>,
  pub chapter: Option<String>,
  pub section_number: Option<String>,
  #[serde(default)]
  pub pages: Vec<i64>,
  #[serde(default)]
  pub citation: String,
  #[serde(default)]
  pub topics: Vec<String>,
  #[serde(default)]
  pub title: String,
  #[serde(default)]
  pub body: String,
  #[serde(default)]
  pub tags: Vec<String>,
}

// --------------------------------------------------------------------------- //
// Annotations (text-anchored highlights)
// --------------------------------------------------------------------------- //

/// A highlight anchored to a quote within a rendered book section.
///
/// Anchoring is resilient to re-render: the client re-locates the range by the
/// `quote` plus a small amount of surrounding context and the `occurrence`
/// index (which copy of an identical quote within the section).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
  #[serde(
    alias = "_id",
    skip_serializing_if = "Option::is_none",
    serialize_with = "ser_oid"
  )]
  pub id: Option<ObjectId>,
  #[serde(default)]
  pub user_id: String,
  pub book_key: String,
  /// Route path of the section.
  pub section_path: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub section_title: Option<String>,
  /// Chapter title of the source section.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub chapter: Option<String>,
  /// Section number (e.g. "2.1").
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub section_number: Option<String>,
  /// Page range `[start, end]` in the source PDF.
  #[serde(default)]
  pub pages: Vec<i64>,
  /// Formatted citation, e.g. "CLRS §2.1, pp. 17–21".
  #[serde(default)]
  pub citation: String,
  /// Topic tags for grouping/filtering.
  #[serde(default)]
  pub topics: Vec<String>,
  /// The exact highlighted text (may contain `$…$` math).
  pub quote: String,
  /// A few characters immediately before the quote (disambiguation).
  #[serde(default)]
  pub prefix: String,
  /// A few characters immediately after the quote (disambiguation).
  #[serde(default)]
  pub suffix: String,
  /// Which occurrence of an identical quote within the section (0-based).
  #[serde(default)]
  pub occurrence: i64,
  /// Highlight color key (`yellow` | `green` | `blue` | `purple` | `orange`).
  #[serde(default = "default_color")]
  pub color: String,
  /// Optional attached note text.
  #[serde(default)]
  pub note: String,
  #[serde(default)]
  pub created_time: String,
  #[serde(default)]
  pub updated_time: String,
}

fn default_color() -> String {
  "yellow".to_string()
}

/// Client-supplied annotation fields (for create/update).
#[derive(Debug, Deserialize)]
pub struct AnnotationInput {
  pub id: Option<String>,
  pub book_key: String,
  pub section_path: String,
  pub section_title: Option<String>,
  pub chapter: Option<String>,
  pub section_number: Option<String>,
  #[serde(default)]
  pub pages: Vec<i64>,
  #[serde(default)]
  pub citation: String,
  #[serde(default)]
  pub topics: Vec<String>,
  pub quote: String,
  #[serde(default)]
  pub prefix: String,
  #[serde(default)]
  pub suffix: String,
  #[serde(default)]
  pub occurrence: i64,
  #[serde(default = "default_color")]
  pub color: String,
  #[serde(default)]
  pub note: String,
}

// --------------------------------------------------------------------------- //
// Progress
// --------------------------------------------------------------------------- //

/// Per-section reading progress for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
  #[serde(
    alias = "_id",
    skip_serializing_if = "Option::is_none",
    serialize_with = "ser_oid"
  )]
  pub id: Option<ObjectId>,
  #[serde(default)]
  pub user_id: String,
  pub book_key: String,
  pub section_path: String,
  /// `reading` | `done`.
  pub status: String,
  /// Furthest reading scroll position on the page, 0.0..1.0.
  #[serde(default)]
  pub scroll: f64,
  #[serde(default)]
  pub updated_time: String,
}

#[derive(Debug, Deserialize)]
pub struct ProgressInput {
  pub book_key: String,
  pub section_path: String,
  /// empty string ⇒ leave the existing status untouched (e.g. scroll-only update)
  #[serde(default)]
  pub status: String,
  /// `None` ⇒ leave the existing scroll untouched (e.g. status-only update)
  #[serde(default)]
  pub scroll: Option<f64>,
}

// --------------------------------------------------------------------------- //
// WebSocket protocol
// --------------------------------------------------------------------------- //

/// Incoming study WebSocket message, discriminated by `"action"`.
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum StudyRequest {
  ListNotes,
  SaveNote { note: NoteInput },
  DeleteNote { id: String },
  ListAnnotations,
  SaveAnnotation { annotation: AnnotationInput },
  DeleteAnnotation { id: String },
  ListProgress,
  SaveProgress { progress: ProgressInput },
}

impl StudyRequest {
  pub fn parse(text: &str) -> Result<Self, String> {
    serde_json::from_str(text).map_err(|e| format!("invalid study request: {e}"))
  }
}

/// Outgoing study WebSocket message, discriminated by `"type"`.
/// Carries `scope: "study"` so the client can route it.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StudyResponse {
  Notes { notes: Vec<Note> },
  NoteSaved { note: Note },
  NoteDeleted { id: String },
  Annotations { annotations: Vec<Annotation> },
  AnnotationSaved { annotation: Annotation },
  AnnotationDeleted { id: String },
  Progress { items: Vec<Progress> },
  ProgressSaved { item: Progress },
  Error { message: String },
}

/// A study mutation broadcast to a user's other live sessions.
#[derive(Debug, Clone)]
pub struct StudyEvent {
  pub user_id: String,
  pub response: StudyResponse,
}
