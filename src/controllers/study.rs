//! # Study Controller
//!
//! Auth (Argon2 password hashing + JWT) and CRUD for the study network's
//! notes, annotations, and reading progress. All collections are namespaced
//! with a `study_` prefix and are independent of the blog.

use crate::{
  db,
  models::study::{
    Annotation, AnnotationInput, Claims, Note, NoteInput, Progress,
    ProgressInput, User,
  },
};

use argon2::{
  Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
  password_hash::SaltString,
};
use chrono::{Duration, Utc};
use futures::TryStreamExt;
use jsonwebtoken::{
  Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use mongodb::{
  Client,
  bson::{doc, oid::ObjectId},
};

const USERS: &str = "study_users";
const NOTES: &str = "study_notes";
const ANNOTATIONS: &str = "study_annotations";
const PROGRESS: &str = "study_progress";

// --------------------------------------------------------------------------- //
// Password hashing
// --------------------------------------------------------------------------- //

pub fn hash_password(password: &str) -> Result<String, String> {
  // 16 random bytes -> b64 salt (avoids pulling in a feature-gated OsRng).
  let salt_bytes: [u8; 16] = rand::random();
  let salt = SaltString::encode_b64(&salt_bytes)
    .map_err(|e| format!("salt error: {e}"))?;
  Argon2::default()
    .hash_password(password.as_bytes(), &salt)
    .map(|h| h.to_string())
    .map_err(|e| format!("hash error: {e}"))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
  match PasswordHash::new(hash) {
    Ok(parsed) => Argon2::default()
      .verify_password(password.as_bytes(), &parsed)
      .is_ok(),
    Err(_) => false,
  }
}

// --------------------------------------------------------------------------- //
// JWT
// --------------------------------------------------------------------------- //

pub fn make_token(secret: &str, user: &User) -> Result<String, String> {
  let exp = (Utc::now() + Duration::days(30)).timestamp() as usize;
  let claims = Claims {
    sub: user.id.map(|o| o.to_hex()).unwrap_or_default(),
    username: user.username.clone(),
    exp,
  };
  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_bytes()),
  )
  .map_err(|e| format!("token error: {e}"))
}

pub fn verify_token(secret: &str, token: &str) -> Result<Claims, String> {
  decode::<Claims>(
    token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::new(Algorithm::HS256),
  )
  .map(|data| data.claims)
  .map_err(|e| format!("invalid token: {e}"))
}

// --------------------------------------------------------------------------- //
// Users
// --------------------------------------------------------------------------- //

pub async fn create_user(
  client: &Client,
  username: &str,
  email: &str,
  password: &str,
) -> Result<User, String> {
  let coll = db::collection::<User>(client, USERS);
  let username = username.trim();
  let email = email.trim().to_lowercase();
  if username.len() < 2 {
    return Err("username too short".into());
  }
  if password.len() < 6 {
    return Err("password must be at least 6 characters".into());
  }

  // reject duplicates (case-insensitive username / email)
  let existing = coll
    .find_one(doc! { "$or": [
      { "username": username },
      { "email": &email },
    ] })
    .await
    .map_err(|e| e.to_string())?;
  if existing.is_some() {
    return Err("username or email already taken".into());
  }

  let mut user = User {
    id: None,
    username: username.to_string(),
    email,
    password_hash: hash_password(password)?,
    created_time: Utc::now().to_rfc3339(),
  };
  let res = coll.insert_one(&user).await.map_err(|e| e.to_string())?;
  user.id = res.inserted_id.as_object_id();
  Ok(user)
}

pub async fn find_user(
  client: &Client,
  identifier: &str,
) -> Result<Option<User>, String> {
  let coll = db::collection::<User>(client, USERS);
  let identifier = identifier.trim();
  coll
    .find_one(doc! { "$or": [
      { "username": identifier },
      { "email": identifier.to_lowercase() },
    ] })
    .await
    .map_err(|e| e.to_string())
}

// --------------------------------------------------------------------------- //
// Notes
// --------------------------------------------------------------------------- //

pub async fn list_notes(
  client: &Client,
  user_id: &str,
) -> Result<Vec<Note>, String> {
  let coll = db::collection::<Note>(client, NOTES);
  coll
    .find(doc! { "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?
    .try_collect()
    .await
    .map_err(|e| e.to_string())
}

pub async fn save_note(
  client: &Client,
  user_id: &str,
  input: NoteInput,
) -> Result<Note, String> {
  let coll = db::collection::<Note>(client, NOTES);
  let now = Utc::now().to_rfc3339();

  if let Some(id) = input.id.as_deref().filter(|s| !s.is_empty()) {
    // update existing (scoped to owner)
    let oid = db::parse_oid(id)?;
    coll
      .update_one(
        doc! { "_id": oid, "user_id": user_id },
        doc! { "$set": {
          "title": &input.title,
          "body": &input.body,
          "tags": &input.tags,
          "book_key": &input.book_key,
          "section_path": &input.section_path,
          "section_title": &input.section_title,
          "chapter": &input.chapter,
          "section_number": &input.section_number,
          "pages": &input.pages,
          "citation": &input.citation,
          "topics": &input.topics,
          "updated_time": &now,
        } },
      )
      .await
      .map_err(|e| e.to_string())?;
    return coll
      .find_one(doc! { "_id": oid, "user_id": user_id })
      .await
      .map_err(|e| e.to_string())?
      .ok_or_else(|| "note not found".into());
  }

  let mut note = Note {
    id: None,
    user_id: user_id.to_string(),
    book_key: input.book_key,
    section_path: input.section_path,
    section_title: input.section_title,
    chapter: input.chapter,
    section_number: input.section_number,
    pages: input.pages,
    citation: input.citation,
    topics: input.topics,
    title: input.title,
    body: input.body,
    tags: input.tags,
    created_time: now.clone(),
    updated_time: now,
  };
  let res = coll.insert_one(&note).await.map_err(|e| e.to_string())?;
  note.id = res.inserted_id.as_object_id();
  Ok(note)
}

pub async fn delete_note(
  client: &Client,
  user_id: &str,
  id: &str,
) -> Result<(), String> {
  let coll = db::collection::<Note>(client, NOTES);
  let oid = db::parse_oid(id)?;
  coll
    .delete_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

// --------------------------------------------------------------------------- //
// Annotations
// --------------------------------------------------------------------------- //

pub async fn list_annotations(
  client: &Client,
  user_id: &str,
) -> Result<Vec<Annotation>, String> {
  let coll = db::collection::<Annotation>(client, ANNOTATIONS);
  coll
    .find(doc! { "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?
    .try_collect()
    .await
    .map_err(|e| e.to_string())
}

pub async fn save_annotation(
  client: &Client,
  user_id: &str,
  input: AnnotationInput,
) -> Result<Annotation, String> {
  let coll = db::collection::<Annotation>(client, ANNOTATIONS);
  let now = Utc::now().to_rfc3339();

  if let Some(id) = input.id.as_deref().filter(|s| !s.is_empty()) {
    let oid = db::parse_oid(id)?;
    coll
      .update_one(
        doc! { "_id": oid, "user_id": user_id },
        doc! { "$set": {
          "color": &input.color,
          "note": &input.note,
          "updated_time": &now,
        } },
      )
      .await
      .map_err(|e| e.to_string())?;
    return coll
      .find_one(doc! { "_id": oid, "user_id": user_id })
      .await
      .map_err(|e| e.to_string())?
      .ok_or_else(|| "annotation not found".into());
  }

  let mut ann = Annotation {
    id: None,
    user_id: user_id.to_string(),
    book_key: input.book_key,
    section_path: input.section_path,
    section_title: input.section_title,
    chapter: input.chapter,
    section_number: input.section_number,
    pages: input.pages,
    citation: input.citation,
    topics: input.topics,
    quote: input.quote,
    prefix: input.prefix,
    suffix: input.suffix,
    occurrence: input.occurrence,
    color: input.color,
    note: input.note,
    created_time: now.clone(),
    updated_time: now,
  };
  let res = coll.insert_one(&ann).await.map_err(|e| e.to_string())?;
  ann.id = res.inserted_id.as_object_id();
  Ok(ann)
}

pub async fn delete_annotation(
  client: &Client,
  user_id: &str,
  id: &str,
) -> Result<(), String> {
  let coll = db::collection::<Annotation>(client, ANNOTATIONS);
  let oid = db::parse_oid(id)?;
  coll
    .delete_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

// --------------------------------------------------------------------------- //
// Progress
// --------------------------------------------------------------------------- //

pub async fn list_progress(
  client: &Client,
  user_id: &str,
) -> Result<Vec<Progress>, String> {
  let coll = db::collection::<Progress>(client, PROGRESS);
  coll
    .find(doc! { "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?
    .try_collect()
    .await
    .map_err(|e| e.to_string())
}

pub async fn save_progress(
  client: &Client,
  user_id: &str,
  input: ProgressInput,
) -> Result<Progress, String> {
  let coll = db::collection::<Progress>(client, PROGRESS);
  let now = Utc::now().to_rfc3339();
  // Upsert by (user, section). `status`/`scroll` are each only written when the
  // caller actually provides one — so a status-only update (mark done / mark
  // reading) leaves scroll alone, and a scroll-only update leaves status alone.
  // Missing fields are seeded with sensible defaults on first insert only.
  let mut set = doc! {
    "user_id": user_id,
    "book_key": &input.book_key,
    "section_path": &input.section_path,
    "updated_time": &now,
  };
  let mut on_insert = doc! {};
  if input.status.is_empty() {
    on_insert.insert("status", "reading");
  } else {
    set.insert("status", &input.status);
  }
  match input.scroll {
    Some(s) => { set.insert("scroll", s); }
    None => { on_insert.insert("scroll", 0.0); }
  }
  let mut update = doc! { "$set": set };
  if !on_insert.is_empty() {
    update.insert("$setOnInsert", on_insert);
  }
  coll
    .update_one(
      doc! { "user_id": user_id, "section_path": &input.section_path },
      update,
    )
    .upsert(true)
    .await
    .map_err(|e| e.to_string())?;

  coll
    .find_one(doc! { "user_id": user_id, "section_path": &input.section_path })
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| "progress not found".into())
}

/// Helper used by `db::parse_oid` callers needing an owned `ObjectId`.
#[allow(dead_code)]
pub fn oid(hex: &str) -> Option<ObjectId> {
  ObjectId::parse_str(hex).ok()
}
