//! Tests for comment request/response protocol
//! serialization and deserialization.

use super::*;
use serde_json;

fn minimal_comment_json() -> serde_json::Value {
  serde_json::json!({
    "text": "hi",
    "markup": "<p>hi</p>",
    "author": "Alice",
    "created_time": "2025-01-01T00:00:00Z",
    "path": "/blog/test",
  })
}

#[test]
fn deserialize_create_request() {
  let json = serde_json::json!({
    "action": "create",
    "comment": minimal_comment_json(),
    "reply_to": null,
  });
  let req: CommentRequest = serde_json::from_value(json).unwrap();
  assert!(matches!(req, CommentRequest::Create { .. }));
}

#[test]
fn deserialize_edit_request() {
  let json = serde_json::json!({
    "action": "edit",
    "id": "abc123",
    "text": "updated",
  });
  let req: CommentRequest = serde_json::from_value(json).unwrap();
  assert!(matches!(
    req,
    CommentRequest::Edit { id, .. } if id == "abc123"
  ));
}

#[test]
fn deserialize_like_request() {
  let json = serde_json::json!({
    "action": "like",
    "id": "abc123",
  });
  let req: CommentRequest = serde_json::from_value(json).unwrap();
  assert!(matches!(
    req,
    CommentRequest::Like { id } if id == "abc123"
  ));
}

#[test]
fn deserialize_delete_request() {
  let json = serde_json::json!({
    "action": "delete",
    "id": "abc123",
  });
  let req: CommentRequest = serde_json::from_value(json).unwrap();
  assert!(matches!(
    req,
    CommentRequest::Delete { id } if id == "abc123"
  ));
}

#[test]
fn deserialize_list_request() {
  let json = serde_json::json!({
    "action": "list",
    "path": "/blog/test",
    "actor": null,
  });
  let req: CommentRequest = serde_json::from_value(json).unwrap();
  assert!(matches!(req, CommentRequest::List { .. }));
}

#[test]
fn invalid_action_fails() {
  let json = serde_json::json!({
    "action": "explode",
    "id": "x",
  });
  let result = serde_json::from_value::<CommentRequest>(json);
  assert!(result.is_err());
}

#[test]
fn comment_edit_partial_fields() {
  let json = serde_json::json!({ "text": "new text" });
  let edit: CommentEdit = serde_json::from_value(json).unwrap();
  assert_eq!(edit.text.as_deref(), Some("new text"));
  assert!(edit.created_time.is_none());

  let empty: CommentEdit =
    serde_json::from_value(serde_json::json!({})).unwrap();
  assert!(empty.text.is_none());
  assert!(empty.created_time.is_none());
}

#[test]
fn serialize_created_response_has_type_tag() {
  let comment = BlogComment {
    id: None,
    text: "hi".into(),
    markup: "<p>hi</p>".into(),
    author: "A".into(),
    created_time: "".into(),
    edited_time: None,
    path: "/".into(),
    likes: 0,
    is_private: None,
    reply_to: None,
    replies: vec![],
  };
  let resp = CommentResponse::Created { comment };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "created");
  assert!(json.get("comment").is_some());
}

#[test]
fn serialize_deleted_response_has_type_tag() {
  let resp = CommentResponse::Deleted {
    id: "abc".into(),
    deleted_count: 3,
  };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "deleted");
  assert_eq!(json["id"], "abc");
  assert_eq!(json["deleted_count"], 3);
}

#[test]
fn serialize_list_response_has_type_tag() {
  let resp = CommentResponse::List { comments: vec![] };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "list");
  assert!(json["comments"].as_array().unwrap().is_empty());
}

#[test]
fn serialize_error_response() {
  let resp = CommentResponse::Error {
    message: "something went wrong".into(),
  };
  let json = serde_json::to_value(&resp).unwrap();
  assert_eq!(json["type"], "error");
  assert_eq!(json["message"], "something went wrong");
}
