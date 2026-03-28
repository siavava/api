//! Tests for comment WebSocket handler dispatch
//! using mock DB.

use super::*;
use crate::{
  controllers::comments::CommentOps,
  models::comments::{CommentRequest, CommentResponse},
  testutil::{MockCommentStore, make_comment},
};

// ---- handle_message ---------------------------------------------------------

#[tokio::test]
async fn handle_message_invalid_json_returns_error() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let (resp, broadcast) =
    handle_message(&db, "not json!", &mut active_route).await;

  assert!(matches!(resp, CommentResponse::Error { .. }));
  assert!(broadcast.is_none());
}

#[tokio::test]
async fn handle_message_valid_create_json() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let json = concat!(
    r#"{"action":"create","comment":"#,
    r#"{"text":"hi","markup":"<p>hi</p>","#,
    r#""author":"alice","path":"/blog/test","#,
    r#""created_time":""}}"#,
  );
  let (resp, broadcast) = handle_message(&db, json, &mut active_route).await;

  assert!(matches!(resp, CommentResponse::Created { .. }));
  assert_eq!(broadcast, Some("/blog/test".into()));
}

// ---- handle_request: Create -------------------------------------------------

#[tokio::test]
async fn create_comment_assigns_id_and_broadcasts() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let comment = make_comment("/blog/a", "hello", "bob");
  let req = CommentRequest::Create {
    comment,
    reply_to: None,
  };

  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Created { comment } => {
      assert!(comment.id.is_some(), "created comment should have an id");
      assert_eq!(comment.text, "hello");
      assert_eq!(comment.author, "bob");
    }
    other => panic!("expected Created, got {other:?}"),
  }
  assert_eq!(broadcast, Some("/blog/a".into()));
}

#[tokio::test]
async fn create_reply_links_to_parent() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  // Seed a parent comment.
  let parent = make_comment("/blog/a", "parent", "alice");
  let parent = db.create_comment(parent, None).await.unwrap();
  let parent_id = parent.id.unwrap().to_hex();

  // Create a reply.
  let reply = make_comment("/blog/a", "reply", "bob");
  let req = CommentRequest::Create {
    comment: reply,
    reply_to: Some(parent_id.clone()),
  };

  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Created { comment } => {
      assert_eq!(comment.reply_to.as_deref(), Some(parent_id.as_str()));
    }
    other => panic!("expected Created, got {other:?}"),
  }
  assert_eq!(broadcast, Some("/blog/a".into()));
}

#[tokio::test]
async fn create_reply_with_invalid_reply_to_returns_error() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let comment = make_comment("/blog/a", "reply", "bob");
  let req = CommentRequest::Create {
    comment,
    reply_to: Some("not-a-valid-oid".into()),
  };

  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  assert!(matches!(resp, CommentResponse::Error { .. }));
  assert!(broadcast.is_none());
}

// ---- handle_request: Edit ---------------------------------------------------

#[tokio::test]
async fn edit_comment_returns_updated_and_broadcasts() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let comment = make_comment("/blog/a", "original", "alice");
  let created = db.create_comment(comment, None).await.unwrap();
  let id = created.id.unwrap().to_hex();

  let req = CommentRequest::Edit {
    id,
    edit: crate::models::comments::CommentEdit {
      text: Some("edited".into()),
      created_time: None,
    },
  };

  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Updated { comment } => {
      assert_eq!(comment.text, "edited");
      assert!(comment.edited_time.is_some());
    }
    other => panic!("expected Updated, got {other:?}"),
  }
  assert_eq!(broadcast, Some("/blog/a".into()));
}

#[tokio::test]
async fn edit_nonexistent_comment_returns_error() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let fake_id = mongodb::bson::oid::ObjectId::new().to_hex();
  let req = CommentRequest::Edit {
    id: fake_id,
    edit: crate::models::comments::CommentEdit {
      text: Some("nope".into()),
      created_time: None,
    },
  };

  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Error { message } => {
      assert!(message.contains("not found"), "got: {message}");
    }
    other => panic!("expected Error, got {other:?}"),
  }
  assert!(broadcast.is_none());
}

// ---- handle_request: Like ---------------------------------------------------

#[tokio::test]
async fn like_comment_increments_and_broadcasts() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let comment = make_comment("/blog/a", "likeable", "alice");
  let created = db.create_comment(comment, None).await.unwrap();
  let id = created.id.unwrap().to_hex();

  let req = CommentRequest::Like { id };
  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Liked { comment } => {
      assert_eq!(comment.likes, 1);
    }
    other => panic!("expected Liked, got {other:?}"),
  }
  assert_eq!(broadcast, Some("/blog/a".into()));
}

#[tokio::test]
async fn like_nonexistent_comment_returns_error() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let fake_id = mongodb::bson::oid::ObjectId::new().to_hex();
  let req = CommentRequest::Like { id: fake_id };
  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Error { message } => {
      assert!(message.contains("not found"), "got: {message}");
    }
    other => panic!("expected Error, got {other:?}"),
  }
  assert!(broadcast.is_none());
}

// ---- handle_request: Delete -------------------------------------------------

#[tokio::test]
async fn delete_comment_returns_count_and_broadcasts() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let comment = make_comment("/blog/a", "doomed", "alice");
  let created = db.create_comment(comment, None).await.unwrap();
  let id = created.id.unwrap().to_hex();

  let req = CommentRequest::Delete { id: id.clone() };
  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Deleted {
      id: deleted_id,
      deleted_count,
    } => {
      assert_eq!(deleted_id, id);
      assert!(deleted_count >= 1);
    }
    other => panic!("expected Deleted, got {other:?}"),
  }
  assert_eq!(broadcast, Some("/blog/a".into()));
}

#[tokio::test]
async fn delete_nonexistent_comment_returns_error() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let fake_id = mongodb::bson::oid::ObjectId::new().to_hex();
  let req = CommentRequest::Delete { id: fake_id };
  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::Error { message } => {
      assert!(message.contains("not found"), "got: {message}");
    }
    other => panic!("expected Error, got {other:?}"),
  }
  assert!(broadcast.is_none());
}

#[tokio::test]
async fn delete_with_invalid_id_returns_error() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let req = CommentRequest::Delete {
    id: "bad-id".into(),
  };
  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  assert!(matches!(resp, CommentResponse::Error { .. }));
  assert!(broadcast.is_none());
}

// ---- handle_request: List ---------------------------------------------------

#[tokio::test]
async fn list_returns_comments_and_sets_active_route() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  // Seed two comments on the same path.
  let c1 = make_comment("/blog/a", "first", "alice");
  let c2 = make_comment("/blog/a", "second", "bob");
  db.create_comment(c1, None).await.unwrap();
  db.create_comment(c2, None).await.unwrap();

  let req = CommentRequest::List {
    path: "/blog/a".into(),
    actor: None,
  };
  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::List { comments } => {
      assert_eq!(comments.len(), 2);
    }
    other => panic!("expected List, got {other:?}"),
  }
  assert!(broadcast.is_none());
  assert_eq!(active_route, Some("/blog/a".into()));
}

#[tokio::test]
async fn list_empty_path_returns_empty() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  let req = CommentRequest::List {
    path: "/blog/empty".into(),
    actor: None,
  };
  let (resp, broadcast) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::List { comments } => {
      assert!(comments.is_empty());
    }
    other => panic!("expected List, got {other:?}"),
  }
  assert!(broadcast.is_none());
}

#[tokio::test]
async fn list_filters_private_comments_for_non_author() {
  let db = MockCommentStore::new();
  let mut active_route = None;

  // Public comment.
  let public = make_comment("/blog/a", "public", "alice");
  db.create_comment(public, None).await.unwrap();

  // Private comment by bob.
  let mut private = make_comment("/blog/a", "secret", "bob");
  private.is_private = Some(true);
  db.create_comment(private, None).await.unwrap();

  // List as alice -- should not see bob's private comment.
  let req = CommentRequest::List {
    path: "/blog/a".into(),
    actor: Some("alice".into()),
  };
  let (resp, _) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::List { comments } => {
      assert_eq!(
        comments.len(),
        1,
        "alice should only see the public comment"
      );
      assert_eq!(comments[0].author, "alice");
    }
    other => panic!("expected List, got {other:?}"),
  }

  // List as bob -- should see both.
  let req = CommentRequest::List {
    path: "/blog/a".into(),
    actor: Some("bob".into()),
  };
  let (resp, _) = handle_request(&db, req, &mut active_route).await;

  match resp {
    CommentResponse::List { comments } => {
      assert_eq!(comments.len(), 2, "bob should see his own private comment");
    }
    other => panic!("expected List, got {other:?}"),
  }
}
