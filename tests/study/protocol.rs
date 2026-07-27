//! Tests for study WebSocket protocol parsing, the
//! anonymous-session auth boundary, and response
//! serialization.

use mongodb::bson::oid::ObjectId;
use server::models::study::{
  Annotation, Note, Progress, Reply, StudyRequest, StudyResponse,
};

fn parse(json: &str) -> StudyRequest {
  StudyRequest::parse(json).unwrap()
}

// ---- request parsing --------------------------------------------------------

#[test]
fn parse_read_actions() {
  assert!(matches!(
    parse(r#"{"action":"list_notes"}"#),
    StudyRequest::ListNotes
  ));
  assert!(matches!(
    parse(r#"{"action":"list_annotations"}"#),
    StudyRequest::ListAnnotations
  ));
  assert!(matches!(
    parse(r#"{"action":"list_progress"}"#),
    StudyRequest::ListProgress
  ));
}

#[test]
fn parse_subscribe_section() {
  match parse(
    r#"{"action":"subscribe_section","section_path":"/books/clrs/2.1"}"#,
  ) {
    StudyRequest::SubscribeSection { section_path } => {
      assert_eq!(section_path, "/books/clrs/2.1");
    }
    other => panic!("expected SubscribeSection, got {other:?}"),
  }
}

#[test]
fn parse_save_note_with_minimal_input() {
  let json = r#"{"action":"save_note","note":{"title":"t","body":"b"}}"#;
  match parse(json) {
    StudyRequest::SaveNote { note } => {
      assert_eq!(note.title, "t");
      assert_eq!(note.body, "b");
      assert!(note.id.is_none());
      assert!(note.book_key.is_none());
      assert!(note.pages.is_empty());
      assert!(note.topics.is_empty());
      assert!(note.tags.is_empty());
      assert_eq!(note.public, None, "visibility defaults to absent");
    }
    other => panic!("expected SaveNote, got {other:?}"),
  }
}

#[test]
fn parse_save_annotation_requires_anchor_fields() {
  let missing = r#"{"action":"save_annotation","annotation":{"quote":"q"}}"#;
  assert!(StudyRequest::parse(missing).is_err());

  let json = concat!(
    r#"{"action":"save_annotation","annotation":{"#,
    r#""book_key":"clrs","section_path":"/books/clrs/2.1","#,
    r#""quote":"loop invariant","occurrence":1}}"#,
  );
  match parse(json) {
    StudyRequest::SaveAnnotation { annotation } => {
      assert_eq!(annotation.quote, "loop invariant");
      assert_eq!(annotation.occurrence, 1);
      assert_eq!(annotation.color, "yellow", "color defaults to yellow");
      assert_eq!(annotation.prefix, "");
      assert_eq!(annotation.suffix, "");
    }
    other => panic!("expected SaveAnnotation, got {other:?}"),
  }
}

#[test]
fn parse_save_progress_scroll_and_status_are_optional() {
  let json = concat!(
    r#"{"action":"save_progress","progress":{"#,
    r#""book_key":"clrs","section_path":"/books/clrs/2.1"}}"#,
  );
  match parse(json) {
    StudyRequest::SaveProgress { progress } => {
      assert_eq!(progress.status, "", "absent status means leave untouched");
      assert_eq!(progress.scroll, None, "absent scroll means leave untouched");
    }
    other => panic!("expected SaveProgress, got {other:?}"),
  }

  let json = concat!(
    r#"{"action":"save_progress","progress":{"#,
    r#""book_key":"clrs","section_path":"/books/clrs/2.1","#,
    r#""status":"done","scroll":0.75}}"#,
  );
  match parse(json) {
    StudyRequest::SaveProgress { progress } => {
      assert_eq!(progress.status, "done");
      assert_eq!(progress.scroll, Some(0.75));
    }
    other => panic!("expected SaveProgress, got {other:?}"),
  }
}

#[test]
fn parse_reply_actions() {
  let json = concat!(
    r#"{"action":"save_reply","reply":{"#,
    r#""parent_id":"507f1f77bcf86cd799439011","parent_kind":"note","#,
    r#""section_path":"/books/clrs/2.1","body":"agreed"}}"#,
  );
  match parse(json) {
    StudyRequest::SaveReply { reply } => {
      assert_eq!(reply.parent_kind, "note");
      assert_eq!(reply.body, "agreed");
    }
    other => panic!("expected SaveReply, got {other:?}"),
  }

  match parse(r#"{"action":"like_reply","id":"507f1f77bcf86cd799439011"}"#) {
    StudyRequest::LikeReply { id } => {
      assert_eq!(id, "507f1f77bcf86cd799439011")
    }
    other => panic!("expected LikeReply, got {other:?}"),
  }
}

#[test]
fn parse_delete_actions() {
  assert!(matches!(
    parse(r#"{"action":"delete_note","id":"x"}"#),
    StudyRequest::DeleteNote { .. }
  ));
  assert!(matches!(
    parse(r#"{"action":"delete_annotation","id":"x"}"#),
    StudyRequest::DeleteAnnotation { .. }
  ));
  assert!(matches!(
    parse(r#"{"action":"delete_reply","id":"x"}"#),
    StudyRequest::DeleteReply { .. }
  ));
}

#[test]
fn parse_rejects_unknown_action_and_bad_json() {
  let err = StudyRequest::parse(r#"{"action":"drop_tables"}"#).unwrap_err();
  assert!(err.contains("invalid study request"));
  assert!(StudyRequest::parse("not json!").is_err());
}

// ---- auth boundary ----------------------------------------------------------

#[test]
fn mutations_require_auth() {
  for json in [
    r#"{"action":"save_note","note":{"title":"t"}}"#,
    r#"{"action":"delete_note","id":"x"}"#,
    concat!(
      r#"{"action":"save_annotation","annotation":{"#,
      r#""book_key":"clrs","section_path":"/s","quote":"q"}}"#,
    ),
    r#"{"action":"delete_annotation","id":"x"}"#,
    concat!(
      r#"{"action":"save_progress","progress":{"#,
      r#""book_key":"clrs","section_path":"/s"}}"#,
    ),
    concat!(
      r#"{"action":"save_reply","reply":{"parent_id":"x","#,
      r#""parent_kind":"note","section_path":"/s"}}"#,
    ),
    r#"{"action":"delete_reply","id":"x"}"#,
    r#"{"action":"like_reply","id":"x"}"#,
  ] {
    assert!(parse(json).requires_auth(), "should require auth: {json}");
  }
}

#[test]
fn reads_and_subscriptions_are_anonymous_friendly() {
  for json in [
    r#"{"action":"list_notes"}"#,
    r#"{"action":"list_annotations"}"#,
    r#"{"action":"list_progress"}"#,
    r#"{"action":"subscribe_section","section_path":"/s"}"#,
  ] {
    assert!(!parse(json).requires_auth(), "should be anonymous: {json}");
  }
}

// ---- response serialization -------------------------------------------------

fn make_note() -> Note {
  Note {
    id: Some(ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap()),
    user_id: "u1".into(),
    book_key: Some("clrs".into()),
    section_path: Some("/books/clrs/2.1".into()),
    section_title: None,
    chapter: None,
    section_number: None,
    pages: vec![17, 21],
    citation: "CLRS §2.1, pp. 17–21".into(),
    topics: vec!["sorting".into()],
    title: "loop invariants".into(),
    body: "the invariant holds at initialization".into(),
    tags: vec![],
    public: true,
    author: "alice".into(),
    created_time: String::new(),
    updated_time: String::new(),
  }
}

#[test]
fn serialize_notes_response() {
  let json = serde_json::to_value(StudyResponse::Notes {
    notes: vec![make_note()],
  })
  .unwrap();
  assert_eq!(json["type"], "notes");
  assert_eq!(json["notes"][0]["id"], "507f1f77bcf86cd799439011");
  assert_eq!(json["notes"][0]["title"], "loop invariants");
}

#[test]
fn serialize_note_saved_and_deleted() {
  let json =
    serde_json::to_value(StudyResponse::NoteSaved { note: make_note() })
      .unwrap();
  assert_eq!(json["type"], "note_saved");

  let json =
    serde_json::to_value(StudyResponse::NoteDeleted { id: "abc".into() })
      .unwrap();
  assert_eq!(json["type"], "note_deleted");
  assert_eq!(json["id"], "abc");
}

#[test]
fn serialize_progress_responses() {
  let item = Progress {
    id: None,
    user_id: "u1".into(),
    book_key: "clrs".into(),
    section_path: "/books/clrs/2.1".into(),
    status: "reading".into(),
    scroll: 0.4,
    updated_time: String::new(),
  };
  let json =
    serde_json::to_value(StudyResponse::ProgressSaved { item }).unwrap();
  assert_eq!(json["type"], "progress_saved");
  assert_eq!(json["item"]["scroll"], 0.4);
  assert_eq!(json["item"]["status"], "reading");
  assert!(
    json["item"].get("id").is_none(),
    "unset id should be skipped, not null"
  );
}

#[test]
fn serialize_section_public_snapshot() {
  let json = serde_json::to_value(StudyResponse::SectionPublic {
    section_path: "/books/clrs/2.1".into(),
    annotations: Vec::<Annotation>::new(),
    notes: vec![make_note()],
    replies: Vec::<Reply>::new(),
  })
  .unwrap();
  assert_eq!(json["type"], "section_public");
  assert_eq!(json["section_path"], "/books/clrs/2.1");
  assert!(json["annotations"].as_array().unwrap().is_empty());
  assert_eq!(json["notes"].as_array().unwrap().len(), 1);
  assert!(json["replies"].as_array().unwrap().is_empty());
}

#[test]
fn serialize_reply_deleted_carries_section() {
  let json = serde_json::to_value(StudyResponse::ReplyDeleted {
    id: "abc".into(),
    section_path: "/s".into(),
  })
  .unwrap();
  assert_eq!(json["type"], "reply_deleted");
  assert_eq!(json["section_path"], "/s");
}

#[test]
fn serialize_error_response() {
  let json = serde_json::to_value(StudyResponse::Error {
    message: "authentication required".into(),
  })
  .unwrap();
  assert_eq!(json["type"], "error");
  assert_eq!(json["message"], "authentication required");
}
