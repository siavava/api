//! Tests for comment model serialization and construction.

use super::*;
use mongodb::bson::oid::ObjectId;
use serde_json;

fn sample_comment(id: Option<ObjectId>) -> BlogComment {
  BlogComment {
    id,
    text: "Hello".into(),
    markup: "<p>Hello</p>".into(),
    author: "Alice".into(),
    created_time: "2025-01-01T00:00:00Z".into(),
    edited_time: None,
    path: "/blog/test".into(),
    likes: 0,
    is_private: None,
    reply_to: None,
    replies: vec![],
  }
}

#[test]
fn blog_comment_serializes_id_as_hex_string() {
  let oid = ObjectId::new();
  let comment = sample_comment(Some(oid));
  let json = serde_json::to_value(&comment).unwrap();
  assert_eq!(json["id"], oid.to_hex());
}

#[test]
fn blog_comment_skips_none_id() {
  let comment = sample_comment(None);
  let json = serde_json::to_value(&comment).unwrap();
  assert!(json.get("id").is_none());
}

#[test]
fn blog_comment_deserializes_underscore_id_alias() {
  let oid = ObjectId::new();
  let json = serde_json::json!({
    "_id": oid.to_hex(),
    "text": "hi",
    "markup": "<p>hi</p>",
    "author": "Bob",
    "created_time": "2025-01-01T00:00:00Z",
    "path": "/blog/x",
  });
  let comment: BlogComment = serde_json::from_value(json).unwrap();
  assert!(comment.id.is_some());
}

#[test]
fn serialize_object_ids_produces_array() {
  let comment = BlogComment {
    replies: vec!["aaa".into(), "bbb".into()],
    ..sample_comment(None)
  };
  let json = serde_json::to_value(&comment).unwrap();
  let replies = json["replies"].as_array().unwrap();
  assert_eq!(replies.len(), 2);
  assert_eq!(replies[0], "aaa");
  assert_eq!(replies[1], "bbb");
}

#[test]
fn populated_comment_copies_fields_and_attaches_replies() {
  let oid = ObjectId::new();
  let comment = sample_comment(Some(oid));
  let reply = PopulatedComment::from_comment(sample_comment(None), vec![]);
  let populated = PopulatedComment::from_comment(comment.clone(), vec![reply]);

  assert_eq!(populated.id, comment.id);
  assert_eq!(populated.text, comment.text);
  assert_eq!(populated.author, comment.author);
  assert_eq!(populated.path, comment.path);
  assert_eq!(populated.likes, comment.likes);
  assert_eq!(populated.replies.len(), 1);
}

#[test]
fn populated_comment_serializes_with_nested_replies() {
  let parent = sample_comment(Some(ObjectId::new()));
  let child = PopulatedComment::from_comment(sample_comment(None), vec![]);
  let populated = PopulatedComment::from_comment(parent, vec![child]);
  let json = serde_json::to_value(&populated).unwrap();

  let replies = json["replies"].as_array().unwrap();
  assert_eq!(replies.len(), 1);
  assert_eq!(replies[0]["text"], "Hello");
}

#[test]
fn optional_fields_skipped_when_none() {
  let comment = sample_comment(None);
  let json = serde_json::to_value(&comment).unwrap();
  assert!(json.get("edited_time").is_none());
  assert!(json.get("is_private").is_none());
  assert!(json.get("reply_to").is_none());
}
