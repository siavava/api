//! Test utilities: data factories and mock store implementations.
//!
//! This module is only compiled under `#[cfg(test)]` (declared in
//! `lib.rs` as `#[cfg(test)] mod testutil;`).

use std::sync::Mutex;

use mongodb::bson::oid::ObjectId;

use crate::{
  controllers::{
    comments::CommentOps,
    views::{ViewsIncrement, ViewsOps},
  },
  models::{
    comments::{BlogComment, CommentEdit, PopulatedComment},
    views::PageViews,
  },
};

// ---------------------------------------------------------------------------
// Factory functions
// ---------------------------------------------------------------------------

/// Creates a [`BlogComment`] with sensible defaults.
pub fn make_comment(path: &str, text: &str, author: &str) -> BlogComment {
  BlogComment {
    id: None,
    text: text.into(),
    markup: format!("<p>{text}</p>"),
    author: author.into(),
    created_time: "2025-01-01T00:00:00Z".into(),
    edited_time: None,
    path: path.into(),
    likes: 0,
    is_private: None,
    reply_to: None,
    replies: vec![],
  }
}

/// Creates a [`PageViews`] with the given route and count.
pub fn make_page_views(route: &str, count: u64) -> PageViews {
  PageViews {
    route: route.into(),
    count,
  }
}

// ---------------------------------------------------------------------------
// MockCommentStore
// ---------------------------------------------------------------------------

/// In-memory comment store for unit tests.
pub struct MockCommentStore {
  pub comments: Mutex<Vec<BlogComment>>,
}

impl MockCommentStore {
  pub fn new() -> Self {
    Self {
      comments: Mutex::new(Vec::new()),
    }
  }

  /// Helper: convert a stored `BlogComment` into a `PopulatedComment`,
  /// recursively resolving replies from the given snapshot.
  fn populate(
    store: &[BlogComment],
    comment: &BlogComment,
  ) -> PopulatedComment {
    let populated_replies: Vec<PopulatedComment> = comment
      .replies
      .iter()
      .filter_map(|rid| {
        let oid = ObjectId::parse_str(rid).ok()?;
        store.iter().find(|c| c.id == Some(oid))
      })
      .map(|c| Self::populate(store, c))
      .collect();
    PopulatedComment::from_comment(comment.clone(), populated_replies)
  }
}

impl CommentOps for MockCommentStore {
  async fn create_comment(
    &self,
    comment: BlogComment,
    reply_to: Option<&ObjectId>,
  ) -> Result<BlogComment, String> {
    let new_id = ObjectId::new();
    let comment = BlogComment {
      id: Some(new_id),
      edited_time: None,
      likes: 0,
      reply_to: reply_to.map(|oid| oid.to_hex()),
      replies: vec![],
      ..comment
    };

    let mut store = self.comments.lock().unwrap();

    // If this is a reply, push our id into the parent's replies.
    if let Some(parent_oid) = reply_to
      && let Some(parent) = store.iter_mut().find(|c| c.id == Some(*parent_oid))
    {
      parent.replies.push(new_id.to_hex());
    }

    store.push(comment.clone());
    Ok(comment)
  }

  async fn edit_comment(
    &self,
    id: &ObjectId,
    edit: CommentEdit,
  ) -> Result<Option<PopulatedComment>, String> {
    let mut store = self.comments.lock().unwrap();
    let comment = match store.iter_mut().find(|c| c.id == Some(*id)) {
      Some(c) => c,
      None => return Ok(None),
    };

    if let Some(ref text) = edit.text {
      comment.text = text.clone();
      comment.markup = format!("<p>{text}</p>");
    }
    comment.edited_time = Some("2025-01-01T00:00:01Z".into());

    let updated = comment.clone();
    let snapshot = store.clone();
    drop(store);
    Ok(Some(Self::populate(&snapshot, &updated)))
  }

  async fn like_comment(
    &self,
    id: &ObjectId,
  ) -> Result<Option<PopulatedComment>, String> {
    let mut store = self.comments.lock().unwrap();
    let comment = match store.iter_mut().find(|c| c.id == Some(*id)) {
      Some(c) => c,
      None => return Ok(None),
    };

    comment.likes += 1;
    let updated = comment.clone();
    let snapshot = store.clone();
    drop(store);
    Ok(Some(Self::populate(&snapshot, &updated)))
  }

  async fn delete_comment(
    &self,
    id: &ObjectId,
  ) -> Result<(u64, Option<String>), String> {
    let mut store = self.comments.lock().unwrap();
    let idx = match store.iter().position(|c| c.id == Some(*id)) {
      Some(i) => i,
      None => return Ok((0, None)),
    };

    let comment = store.remove(idx);
    let path = comment.path.clone();

    // Remove this id from the parent's replies list.
    if let Some(ref parent_id_str) = comment.reply_to
      && let Ok(parent_oid) = ObjectId::parse_str(parent_id_str)
      && let Some(parent) = store.iter_mut().find(|c| c.id == Some(parent_oid))
    {
      parent.replies.retain(|r| r != &id.to_hex());
    }

    // Iteratively collect and remove all nested replies.
    let mut to_delete: Vec<ObjectId> = comment
      .replies
      .iter()
      .filter_map(|r| ObjectId::parse_str(r).ok())
      .collect();
    let mut deleted = 1u64;

    while let Some(oid) = to_delete.pop() {
      if let Some(idx) = store.iter().position(|c| c.id == Some(oid)) {
        let child = store.remove(idx);
        for r in &child.replies {
          if let Ok(rid) = ObjectId::parse_str(r) {
            to_delete.push(rid);
          }
        }
        deleted += 1;
      }
    }

    Ok((deleted, Some(path)))
  }

  async fn list_comments(
    &self,
    path: &str,
    actor: Option<&str>,
  ) -> Result<Vec<PopulatedComment>, String> {
    let store = self.comments.lock().unwrap();
    let top_level: Vec<&BlogComment> = store
      .iter()
      .filter(|c| c.path == path && c.reply_to.is_none())
      .filter(|c| {
        if c.is_private == Some(true) {
          actor.is_some_and(|a| a == c.author)
        } else {
          true
        }
      })
      .collect();

    let populated: Vec<PopulatedComment> = top_level
      .iter()
      .map(|c| Self::populate(&store, c))
      .collect();
    Ok(populated)
  }
}

// ---------------------------------------------------------------------------
// MockViewsStore
// ---------------------------------------------------------------------------

/// In-memory page-views store for unit tests.
pub struct MockViewsStore {
  pub views: Mutex<Vec<PageViews>>,
}

impl MockViewsStore {
  pub fn new() -> Self {
    Self {
      views: Mutex::new(Vec::new()),
    }
  }
}

#[cfg(test)]
mod tests;

impl ViewsOps for MockViewsStore {
  async fn get_views(
    &self,
    route: &str,
    increment: ViewsIncrement,
  ) -> Result<PageViews, String> {
    let mut store = self.views.lock().unwrap();
    if let Some(pv) = store.iter_mut().find(|v| v.route == route) {
      if matches!(increment, ViewsIncrement::INCREMENT) {
        pv.count += 1;
      }
      return Ok(pv.clone());
    }

    let count = match increment {
      ViewsIncrement::INCREMENT => 1,
      ViewsIncrement::NOINCREMENT => 0,
    };
    let pv = PageViews {
      route: route.into(),
      count,
    };
    store.push(pv.clone());
    Ok(pv)
  }

  async fn get_all_views(
    &self,
    namespace: Option<&str>,
  ) -> Result<Vec<PageViews>, String> {
    let mut all = self.views.lock().unwrap().clone();
    if let Some(ns) = namespace {
      let prefix = format!("{ns}:");
      all.retain(|view| view.route.starts_with(&prefix));
    }
    all.sort_by(|a, b| b.count.cmp(&a.count));
    Ok(all)
  }
}
