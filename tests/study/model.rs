//! Tests for study data models: serde defaults, DB
//! round-trips, the public-user projection, and the
//! progress upsert document.

use mongodb::bson::{doc, from_document, oid::ObjectId};
use server::{
  controllers::study::progress_update,
  models::study::{Annotation, Note, ProgressInput, PublicUser, Reply, User},
};

// ---- users ------------------------------------------------------------------

fn make_user() -> User {
  User {
    id: Some(ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap()),
    username: "alice".into(),
    email: "alice@example.com".into(),
    password_hash: "$argon2id$fake".into(),
    created_time: String::new(),
  }
}

#[test]
fn public_user_drops_password_hash() {
  let public = PublicUser::from(&make_user());
  let json = serde_json::to_value(&public).unwrap();
  assert_eq!(json["id"], "507f1f77bcf86cd799439011");
  assert_eq!(json["username"], "alice");
  assert!(json.get("password_hash").is_none());
}

#[test]
fn public_user_from_unsaved_user_has_empty_id() {
  let mut user = make_user();
  user.id = None;
  assert_eq!(PublicUser::from(&user).id, "");
}

#[test]
fn user_deserializes_from_db_document() {
  let oid = ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap();
  let user: User = from_document(doc! {
    "_id": oid,
    "username": "alice",
    "email": "alice@example.com",
    "password_hash": "$argon2id$fake",
  })
  .unwrap();
  assert_eq!(user.id, Some(oid));
  assert_eq!(user.created_time, "", "missing created_time defaults");
}

// ---- notes ------------------------------------------------------------------

#[test]
fn note_deserializes_from_minimal_document() {
  let note: Note = from_document(doc! {}).unwrap();
  assert!(note.id.is_none());
  assert!(note.book_key.is_none());
  assert!(note.pages.is_empty());
  assert!(!note.public, "visibility defaults to private");
  assert_eq!(note.author, "");
}

#[test]
fn note_id_serializes_as_hex_string() {
  let oid = ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap();
  let note: Note = from_document(doc! { "_id": oid }).unwrap();
  let json = serde_json::to_value(&note).unwrap();
  assert_eq!(json["id"], "507f1f77bcf86cd799439011");
}

#[test]
fn note_skips_unset_optional_fields() {
  let note: Note = from_document(doc! {}).unwrap();
  let json = serde_json::to_value(&note).unwrap();
  for key in ["id", "book_key", "section_path", "section_title", "chapter"] {
    assert!(
      json.get(key).is_none(),
      "{key} should be skipped when unset"
    );
  }
}

// ---- annotations ------------------------------------------------------------

#[test]
fn annotation_color_defaults_to_yellow() {
  let ann: Annotation = from_document(doc! {
    "book_key": "clrs",
    "section_path": "/books/clrs/2.1",
    "quote": "loop invariant",
  })
  .unwrap();
  assert_eq!(ann.color, "yellow");
  assert_eq!(ann.occurrence, 0);
  assert!(!ann.public);
}

#[test]
fn annotation_requires_anchor_fields() {
  let missing_quote: Result<Annotation, _> = from_document(doc! {
    "book_key": "clrs",
    "section_path": "/books/clrs/2.1",
  });
  assert!(missing_quote.is_err(), "quote has no default");
}

// ---- replies ----------------------------------------------------------------

#[test]
fn reply_defaults_to_zero_likes() {
  let reply: Reply = from_document(doc! {
    "parent_id": "507f1f77bcf86cd799439011",
    "parent_kind": "annotation",
    "section_path": "/books/clrs/2.1",
  })
  .unwrap();
  assert_eq!(reply.likes, 0);
  assert!(reply.liked_by.is_empty());
  assert_eq!(reply.body, "");
}

// ---- progress upsert --------------------------------------------------------

fn progress_input(status: &str, scroll: Option<f64>) -> ProgressInput {
  ProgressInput {
    book_key: "clrs".into(),
    section_path: "/books/clrs/2.1".into(),
    status: status.into(),
    scroll,
  }
}

#[test]
fn progress_update_with_both_fields_sets_both() {
  let update = progress_update("u1", &progress_input("done", Some(0.9)), "now");
  let set = update.get_document("$set").unwrap();
  assert_eq!(set.get_str("status").unwrap(), "done");
  assert_eq!(set.get_f64("scroll").unwrap(), 0.9);
  assert!(
    update.get_document("$setOnInsert").is_err(),
    "no insert-only seeds when both fields are provided"
  );
}

#[test]
fn progress_update_scroll_only_leaves_status_untouched() {
  let update = progress_update("u1", &progress_input("", Some(0.5)), "now");
  let set = update.get_document("$set").unwrap();
  assert!(
    set.get("status").is_none(),
    "status must not be overwritten"
  );
  assert_eq!(set.get_f64("scroll").unwrap(), 0.5);
  let seed = update.get_document("$setOnInsert").unwrap();
  assert_eq!(
    seed.get_str("status").unwrap(),
    "reading",
    "first insert seeds status"
  );
}

#[test]
fn progress_update_status_only_leaves_scroll_untouched() {
  let update = progress_update("u1", &progress_input("done", None), "now");
  let set = update.get_document("$set").unwrap();
  assert_eq!(set.get_str("status").unwrap(), "done");
  assert!(
    set.get("scroll").is_none(),
    "scroll must not be overwritten"
  );
  let seed = update.get_document("$setOnInsert").unwrap();
  assert_eq!(
    seed.get_f64("scroll").unwrap(),
    0.0,
    "first insert seeds scroll"
  );
}

#[test]
fn progress_update_always_writes_identity_fields() {
  let update = progress_update("u1", &progress_input("", None), "stamp");
  let set = update.get_document("$set").unwrap();
  assert_eq!(set.get_str("user_id").unwrap(), "u1");
  assert_eq!(set.get_str("book_key").unwrap(), "clrs");
  assert_eq!(set.get_str("section_path").unwrap(), "/books/clrs/2.1");
  assert_eq!(set.get_str("updated_time").unwrap(), "stamp");
}
