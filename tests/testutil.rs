//! Tests for mock DB implementations and test data
//! factories.

mod common;

use common::{MockCommentStore, MockViewsStore, make_comment, make_page_views};
use mongodb::bson::oid::ObjectId;
use server::{
  controllers::{
    comments::CommentOps,
    views::{ViewsIncrement, ViewsOps},
  },
  models::comments::CommentEdit,
};

// ---------------------------------------------------------------------------
// MockCommentStore tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_assigns_id() {
  let store = MockCommentStore::new();
  let c = make_comment("/blog/a", "hello", "alice");
  let created = store.create_comment(c, None).await.unwrap();
  assert!(created.id.is_some(), "created comment should have an id");
}

#[tokio::test]
async fn create_as_reply_links_parent() {
  let store = MockCommentStore::new();
  let parent = store
    .create_comment(make_comment("/blog/a", "parent", "alice"), None)
    .await
    .unwrap();
  let parent_id = parent.id.unwrap();

  let child = store
    .create_comment(make_comment("/blog/a", "reply", "bob"), Some(&parent_id))
    .await
    .unwrap();
  let child_id = child.id.unwrap();

  let comments = store.comments.lock().unwrap();
  let parent = comments.iter().find(|c| c.id == Some(parent_id)).unwrap();
  assert!(
    parent.replies.contains(&child_id.to_hex()),
    "parent replies should contain child id"
  );
}

#[tokio::test]
async fn edit_updates_text() {
  let store = MockCommentStore::new();
  let c = store
    .create_comment(make_comment("/blog/a", "original", "alice"), None)
    .await
    .unwrap();
  let id = c.id.unwrap();

  let edited = store
    .edit_comment(
      &id,
      CommentEdit {
        text: Some("updated".into()),
        created_time: None,
      },
    )
    .await
    .unwrap();

  let edited = edited.expect("edit should return Some");
  assert_eq!(edited.text, "updated");
  assert!(edited.edited_time.is_some(), "edited_time should be set");
}

#[tokio::test]
async fn edit_nonexistent_returns_none() {
  let store = MockCommentStore::new();
  let bogus = ObjectId::new();
  let result = store
    .edit_comment(
      &bogus,
      CommentEdit {
        text: Some("nope".into()),
        created_time: None,
      },
    )
    .await
    .unwrap();
  assert!(result.is_none());
}

#[tokio::test]
async fn like_increments_count() {
  let store = MockCommentStore::new();
  let c = store
    .create_comment(make_comment("/blog/a", "likeable", "alice"), None)
    .await
    .unwrap();
  let id = c.id.unwrap();
  assert_eq!(c.likes, 0);

  let liked = store.like_comment(&id).await.unwrap().unwrap();
  assert_eq!(liked.likes, 1);
}

#[tokio::test]
async fn like_nonexistent_returns_none() {
  let store = MockCommentStore::new();
  let bogus = ObjectId::new();
  let result = store.like_comment(&bogus).await.unwrap();
  assert!(result.is_none());
}

#[tokio::test]
async fn delete_removes_comment() {
  let store = MockCommentStore::new();
  let c = store
    .create_comment(make_comment("/blog/a", "doomed", "alice"), None)
    .await
    .unwrap();
  let id = c.id.unwrap();

  let (count, path) = store.delete_comment(&id).await.unwrap();
  assert_eq!(count, 1);
  assert_eq!(path, Some("/blog/a".into()));

  let comments = store.comments.lock().unwrap();
  assert!(!comments.iter().any(|c| c.id == Some(id)));
}

#[tokio::test]
async fn delete_nonexistent() {
  let store = MockCommentStore::new();
  let bogus = ObjectId::new();
  let (count, path) = store.delete_comment(&bogus).await.unwrap();
  assert_eq!(count, 0);
  assert!(path.is_none());
}

#[tokio::test]
async fn delete_cascades_to_replies() {
  let store = MockCommentStore::new();
  let parent = store
    .create_comment(make_comment("/blog/a", "parent", "alice"), None)
    .await
    .unwrap();
  let parent_id = parent.id.unwrap();

  let child = store
    .create_comment(make_comment("/blog/a", "child", "bob"), Some(&parent_id))
    .await
    .unwrap();
  let child_id = child.id.unwrap();

  let grandchild = store
    .create_comment(
      make_comment("/blog/a", "grandchild", "carol"),
      Some(&child_id),
    )
    .await
    .unwrap();
  let _grandchild_id = grandchild.id.unwrap();

  let (count, path) = store.delete_comment(&parent_id).await.unwrap();
  assert_eq!(count, 3, "parent + child + grandchild should be deleted");
  assert_eq!(path, Some("/blog/a".into()));

  let comments = store.comments.lock().unwrap();
  assert!(comments.is_empty(), "all comments should be removed");
}

#[tokio::test]
async fn list_returns_only_top_level_on_path() {
  let store = MockCommentStore::new();
  let parent = store
    .create_comment(make_comment("/blog/a", "top", "alice"), None)
    .await
    .unwrap();
  let parent_id = parent.id.unwrap();

  store
    .create_comment(make_comment("/blog/a", "reply", "bob"), Some(&parent_id))
    .await
    .unwrap();

  // Different path — should not appear.
  store
    .create_comment(make_comment("/blog/b", "other", "carol"), None)
    .await
    .unwrap();

  let listed = store.list_comments("/blog/a", None).await.unwrap();
  assert_eq!(listed.len(), 1, "only one top-level comment on /blog/a");
  assert_eq!(listed[0].text, "top");
  assert_eq!(listed[0].replies.len(), 1, "reply should be nested inside");
}

#[tokio::test]
async fn list_filters_private_comments_by_actor() {
  let store = MockCommentStore::new();

  // Public comment.
  store
    .create_comment(make_comment("/blog/a", "public", "alice"), None)
    .await
    .unwrap();

  // Private comment by bob.
  let mut private = make_comment("/blog/a", "secret", "bob");
  private.is_private = Some(true);
  store.create_comment(private, None).await.unwrap();

  // No actor — should only see public.
  let listed = store.list_comments("/blog/a", None).await.unwrap();
  assert_eq!(listed.len(), 1);
  assert_eq!(listed[0].text, "public");

  // Actor is bob — should see both.
  let listed = store.list_comments("/blog/a", Some("bob")).await.unwrap();
  assert_eq!(listed.len(), 2);

  // Actor is alice — should only see public (not bob's private).
  let listed = store.list_comments("/blog/a", Some("alice")).await.unwrap();
  assert_eq!(listed.len(), 1);
  assert_eq!(listed[0].text, "public");
}

// ---------------------------------------------------------------------------
// MockViewsStore tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_views_creates_new_with_increment() {
  let store = MockViewsStore::new();
  let pv = store
    .get_views("/page/a", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  assert_eq!(pv.route, "/page/a");
  assert_eq!(pv.count, 1);
}

#[tokio::test]
async fn get_views_creates_new_without_increment() {
  let store = MockViewsStore::new();
  let pv = store
    .get_views("/page/a", ViewsIncrement::NOINCREMENT)
    .await
    .unwrap();
  assert_eq!(pv.route, "/page/a");
  assert_eq!(pv.count, 0);
}

#[tokio::test]
async fn get_views_increments_existing() {
  let store = MockViewsStore::new();
  store
    .get_views("/page/a", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  let pv = store
    .get_views("/page/a", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  assert_eq!(pv.count, 2);
}

#[tokio::test]
async fn get_views_no_increment_on_existing() {
  let store = MockViewsStore::new();
  store
    .get_views("/page/a", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  let pv = store
    .get_views("/page/a", ViewsIncrement::NOINCREMENT)
    .await
    .unwrap();
  assert_eq!(pv.count, 1, "count should remain at 1");
}

#[tokio::test]
async fn get_all_views_sorted_by_count_descending() {
  let store = MockViewsStore::new();

  // Create entries with different counts.
  store
    .get_views("/page/a", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  store
    .get_views("/page/b", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  store
    .get_views("/page/b", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  store
    .get_views("/page/c", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  store
    .get_views("/page/c", ViewsIncrement::INCREMENT)
    .await
    .unwrap();
  store
    .get_views("/page/c", ViewsIncrement::INCREMENT)
    .await
    .unwrap();

  let all = store.get_all_views().await.unwrap();
  assert_eq!(all.len(), 3);
  assert_eq!(all[0].route, "/page/c");
  assert_eq!(all[0].count, 3);
  assert_eq!(all[1].route, "/page/b");
  assert_eq!(all[1].count, 2);
  assert_eq!(all[2].route, "/page/a");
  assert_eq!(all[2].count, 1);
}

// ---------------------------------------------------------------------------
// Factory tests
// ---------------------------------------------------------------------------

#[test]
fn make_comment_has_sensible_defaults() {
  let c = make_comment("/blog/x", "hello", "alice");
  assert!(c.id.is_none(), "factory comment id should be None");
  assert_eq!(c.likes, 0);
  assert!(c.replies.is_empty());
  assert_eq!(c.path, "/blog/x");
  assert_eq!(c.text, "hello");
  assert_eq!(c.author, "alice");
}

#[test]
fn make_page_views_matches() {
  let pv = make_page_views("/page/y", 42);
  assert_eq!(pv.route, "/page/y");
  assert_eq!(pv.count, 42);
}
