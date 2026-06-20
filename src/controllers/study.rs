//! # Study Controller
//!
//! Auth (Argon2 password hashing + JWT) and CRUD for the study network's
//! notes, annotations, and reading progress. All collections are namespaced
//! with a `study_` prefix and are independent of the blog.

use crate::{
  db,
  models::study::{
    Annotation, AnnotationInput, Claims, Note, NoteInput, Progress,
    ProgressInput, Reply, ReplyInput, User,
  },
};

use argon2::{
  Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
  password_hash::SaltString,
};
use chrono::{Duration, Utc};
use futures::TryStreamExt;
use std::collections::HashMap;
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
const REPLIES: &str = "study_replies";

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

/// Saves a note (insert or owner-scoped update). Returns the saved note and
/// whether it was *public before* this save — the route uses that to detect an
/// unpublish and tell other viewers to drop it.
pub async fn save_note(
  client: &Client,
  user_id: &str,
  author: &str,
  input: NoteInput,
) -> Result<(Note, bool), String> {
  let coll = db::collection::<Note>(client, NOTES);
  let now = Utc::now().to_rfc3339();
  let public = input.public.unwrap_or(false);

  if let Some(id) = input.id.as_deref().filter(|s| !s.is_empty()) {
    // update existing (scoped to owner)
    let oid = db::parse_oid(id)?;
    let was_public = coll
      .find_one(doc! { "_id": oid, "user_id": user_id })
      .await
      .map_err(|e| e.to_string())?
      .map(|n| n.public)
      .unwrap_or(false);
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
          "public": public,
          "author": author,
          "updated_time": &now,
        } },
      )
      .await
      .map_err(|e| e.to_string())?;
    let note = coll
      .find_one(doc! { "_id": oid, "user_id": user_id })
      .await
      .map_err(|e| e.to_string())?
      .ok_or_else(|| "note not found".to_string())?;
    return Ok((note, was_public));
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
    public,
    author: author.to_string(),
    created_time: now.clone(),
    updated_time: now,
  };
  let res = coll.insert_one(&note).await.map_err(|e| e.to_string())?;
  note.id = res.inserted_id.as_object_id();
  Ok((note, false))
}

/// Deletes a note (owner-scoped). Returns the deleted note's
/// `(was_public, section_path)` so the route can drop it from other viewers.
pub async fn delete_note(
  client: &Client,
  user_id: &str,
  id: &str,
) -> Result<Option<(bool, Option<String>)>, String> {
  let coll = db::collection::<Note>(client, NOTES);
  let oid = db::parse_oid(id)?;
  let existing = coll
    .find_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  coll
    .delete_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  if let Some(note) = &existing {
    if let Some(section_path) = note.section_path.as_deref() {
      delete_reply_subtree(client, section_path, id).await?;
    }
  }
  Ok(existing.map(|n| (n.public, n.section_path)))
}

/// Cascade-deletes the whole reply subtree under `root_id` (a note, annotation,
/// or reply id): every descendant reply, at any nesting depth, from any author —
/// so deleting a parent never orphans a nested thread. Replies nest purely via
/// `parent_id`, so we walk the tree level by level and delete every id found.
/// Mirrors the blog's recursive comment delete.
async fn delete_reply_subtree(
  client: &Client,
  section_path: &str,
  root_id: &str,
) -> Result<(), String> {
  let coll = db::collection::<Reply>(client, REPLIES);
  // A reply shares its whole ancestry's section, so the entire tree lives in one
  // section. Load it once, then walk in memory — two queries, regardless of depth.
  let all: Vec<Reply> = coll
    .find(doc! { "section_path": section_path })
    .await
    .map_err(|e| e.to_string())?
    .try_collect()
    .await
    .map_err(|e| e.to_string())?;

  let mut children: HashMap<String, Vec<ObjectId>> = HashMap::new();
  for r in &all {
    if let Some(oid) = r.id {
      children.entry(r.parent_id.clone()).or_default().push(oid);
    }
  }

  let mut frontier = vec![root_id.to_string()];
  let mut doomed: Vec<ObjectId> = Vec::new();
  while let Some(parent) = frontier.pop() {
    if let Some(kids) = children.get(&parent) {
      for &oid in kids {
        doomed.push(oid);
        frontier.push(oid.to_hex());
      }
    }
  }

  if !doomed.is_empty() {
    coll
      .delete_many(doc! { "_id": { "$in": doomed } })
      .await
      .map_err(|e| e.to_string())?;
  }
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

/// Saves an annotation (insert or owner-scoped update). Returns the saved
/// annotation and whether it was *public before* this save (for unpublish
/// detection in the route).
pub async fn save_annotation(
  client: &Client,
  user_id: &str,
  author: &str,
  input: AnnotationInput,
) -> Result<(Annotation, bool), String> {
  let coll = db::collection::<Annotation>(client, ANNOTATIONS);
  let now = Utc::now().to_rfc3339();
  let public = input.public.unwrap_or(false);

  if let Some(id) = input.id.as_deref().filter(|s| !s.is_empty()) {
    let oid = db::parse_oid(id)?;
    let was_public = coll
      .find_one(doc! { "_id": oid, "user_id": user_id })
      .await
      .map_err(|e| e.to_string())?
      .map(|a| a.public)
      .unwrap_or(false);
    coll
      .update_one(
        doc! { "_id": oid, "user_id": user_id },
        doc! { "$set": {
          "color": &input.color,
          "note": &input.note,
          "public": public,
          "author": author,
          "updated_time": &now,
        } },
      )
      .await
      .map_err(|e| e.to_string())?;
    let ann = coll
      .find_one(doc! { "_id": oid, "user_id": user_id })
      .await
      .map_err(|e| e.to_string())?
      .ok_or_else(|| "annotation not found".to_string())?;
    return Ok((ann, was_public));
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
    public,
    author: author.to_string(),
    created_time: now.clone(),
    updated_time: now,
  };
  let res = coll.insert_one(&ann).await.map_err(|e| e.to_string())?;
  ann.id = res.inserted_id.as_object_id();
  Ok((ann, false))
}

/// Deletes an annotation (owner-scoped). Returns the deleted annotation's
/// `(was_public, section_path)` so the route can drop it from other viewers.
pub async fn delete_annotation(
  client: &Client,
  user_id: &str,
  id: &str,
) -> Result<Option<(bool, String)>, String> {
  let coll = db::collection::<Annotation>(client, ANNOTATIONS);
  let oid = db::parse_oid(id)?;
  let existing = coll
    .find_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  coll
    .delete_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  if let Some(annotation) = &existing {
    delete_reply_subtree(client, &annotation.section_path, id).await?;
  }
  Ok(existing.map(|a| (a.public, a.section_path)))
}


/// All public annotations + public notes + replies for a section, from every
/// user. Powers the `SubscribeSection` snapshot (and live deltas thereafter).
pub async fn list_public_section(
  client: &Client,
  section_path: &str,
) -> Result<(Vec<Annotation>, Vec<Note>, Vec<Reply>), String> {
  let annotations = db::collection::<Annotation>(client, ANNOTATIONS)
    .find(doc! { "section_path": section_path, "public": true })
    .await
    .map_err(|e| e.to_string())?
    .try_collect()
    .await
    .map_err(|e| e.to_string())?;
  let notes = db::collection::<Note>(client, NOTES)
    .find(doc! { "section_path": section_path, "public": true })
    .await
    .map_err(|e| e.to_string())?
    .try_collect()
    .await
    .map_err(|e| e.to_string())?;
  let replies = list_replies_for_section(client, section_path).await?;
  Ok((annotations, notes, replies))
}

pub async fn list_replies_for_section(
  client: &Client,
  section_path: &str,
) -> Result<Vec<Reply>, String> {
  db::collection::<Reply>(client, REPLIES)
    .find(doc! { "section_path": section_path })
    .await
    .map_err(|e| e.to_string())?
    .try_collect()
    .await
    .map_err(|e| e.to_string())
}

/// Saves a reply (insert, or owner-scoped body edit). Any signed-in user may
/// create one; only the author may edit. `parent_kind` is validated.
pub async fn save_reply(
  client: &Client,
  user_id: &str,
  author: &str,
  input: ReplyInput,
) -> Result<Reply, String> {
  if input.parent_kind != "annotation" && input.parent_kind != "note" {
    return Err("invalid parent_kind".into());
  }
  let coll = db::collection::<Reply>(client, REPLIES);
  let now = Utc::now().to_rfc3339();

  if let Some(id) = input.id.as_deref().filter(|s| !s.is_empty()) {
    let oid = db::parse_oid(id)?;
    coll
      .update_one(
        doc! { "_id": oid, "user_id": user_id },
        doc! { "$set": { "body": &input.body, "updated_time": &now } },
      )
      .await
      .map_err(|e| e.to_string())?;
    return coll
      .find_one(doc! { "_id": oid, "user_id": user_id })
      .await
      .map_err(|e| e.to_string())?
      .ok_or_else(|| "reply not found".into());
  }

  let mut reply = Reply {
    id: None,
    parent_id: input.parent_id,
    parent_kind: input.parent_kind,
    section_path: input.section_path,
    user_id: user_id.to_string(),
    author: author.to_string(),
    body: input.body,
    created_time: now.clone(),
    updated_time: now,
    likes: 0,
    liked_by: Vec::new(),
  };
  let res = coll.insert_one(&reply).await.map_err(|e| e.to_string())?;
  reply.id = res.inserted_id.as_object_id();
  Ok(reply)
}

/// Deletes a reply (owner-scoped). Returns its `section_path` for the broadcast.
pub async fn delete_reply(
  client: &Client,
  user_id: &str,
  id: &str,
) -> Result<String, String> {
  let coll = db::collection::<Reply>(client, REPLIES);
  let oid = db::parse_oid(id)?;
  let existing = coll
    .find_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  coll
    .delete_one(doc! { "_id": oid, "user_id": user_id })
    .await
    .map_err(|e| e.to_string())?;
  if let Some(reply) = &existing {
    delete_reply_subtree(client, &reply.section_path, id).await?;
  }
  Ok(existing.map(|r| r.section_path).unwrap_or_default())
}

/// Toggles `user_id`'s like on a reply and returns the updated record. Any
/// authenticated user may like (not owner-scoped).
pub async fn like_reply(
  client: &Client,
  user_id: &str,
  id: &str,
) -> Result<Reply, String> {
  let coll = db::collection::<Reply>(client, REPLIES);
  let oid = db::parse_oid(id)?;
  let mut reply = coll
    .find_one(doc! { "_id": oid })
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| "reply not found".to_string())?;

  if let Some(pos) = reply.liked_by.iter().position(|u| u == user_id) {
    reply.liked_by.remove(pos);
  } else {
    reply.liked_by.push(user_id.to_string());
  }
  reply.likes = reply.liked_by.len() as i64;

  coll
    .update_one(
      doc! { "_id": oid },
      doc! { "$set": { "liked_by": &reply.liked_by, "likes": reply.likes } },
    )
    .await
    .map_err(|e| e.to_string())?;
  Ok(reply)
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
