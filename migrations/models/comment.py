"""Comment model for migration scripts.

Mirrors the ``BlogComment`` Rust struct (``src/models/comments/model.rs``)
stored in the MongoDB ``comments`` collection.
"""

from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Optional

from bson import ObjectId


@dataclass
class Comment:
  """A single blog comment, matching the ``BlogComment`` Rust struct.

  Attributes:
    text: Raw plain-text content of the comment.
    markup: Pre-rendered markup (e.g. HTML) of the comment.
    author: Display name of the comment author.
    path: Page path the comment belongs to (e.g. ``<b>:/shorts/2026-01-voir-dire``).
    created_time: ISO 8601 timestamp set on creation (server-side in Rust).
    edited_time: ISO 8601 timestamp of last edit, or ``None`` if never edited.
    likes: Non-negative like count.
    is_private: When ``True`` the comment is only visible to its author.
        ``None`` or ``False`` means public.
    reply_to: Hex ``ObjectId`` of the parent comment if this is a reply,
        or ``None`` for top-level comments.
    replies: Hex ``ObjectId`` strings of direct child replies.
    id: MongoDB document ``_id``. ``None`` for comments not yet persisted.
  """

  text: str
  markup: str
  author: str
  path: str
  created_time: str = field(
    default_factory=lambda: datetime.now(timezone.utc).isoformat(),
  )
  edited_time: Optional[str] = None
  likes: int = 0
  is_private: Optional[bool] = None
  reply_to: Optional[str] = None
  replies: list[str] = field(default_factory=list[str])
  id: Optional[ObjectId] = None

  @classmethod
  def from_doc(cls, doc: dict[str, object]) -> Comment:
    """Construct a :class:`Comment` from a raw MongoDB document.

    Args:
      doc: A dictionary as returned by ``pymongo``'s ``find()`` or
          ``find_one()``.

    Returns:
      A populated :class:`Comment` instance.
    """
    return cls(
      id=doc.get("_id"),  # type: ignore[arg-type]
      text=str(doc.get("text", "")),
      markup=str(doc.get("markup", "")),
      author=str(doc.get("author", "")),
      path=str(doc.get("path", "")),
      created_time=str(doc.get("created_time", "")),
      edited_time=str(doc["edited_time"]) if doc.get("edited_time") else None,
      likes=int(doc.get("likes", 0)),  # type: ignore[arg-type]
      is_private=bool(doc["is_private"]) if doc.get("is_private") is not None else None,
      reply_to=str(doc["reply_to"]) if doc.get("reply_to") else None,
      replies=[str(r) for r in doc.get("replies", [])],  # type: ignore[union-attr]
    )

  def to_doc(self) -> dict[str, object]:
    """Convert this comment to a MongoDB-ready document.

    Optional fields that are ``None`` are omitted from the output,
    matching the ``skip_serializing_if`` behaviour in the Rust model.

    Returns:
      A dictionary suitable for ``insert_one()`` or ``replace_one()``.
    """
    doc: dict[str, object] = {
      "text": self.text,
      "markup": self.markup,
      "author": self.author,
      "path": self.path,
      "created_time": self.created_time,
      "likes": self.likes,
      "replies": self.replies,
    }
    if self.id is not None:
      doc["_id"] = self.id
    if self.edited_time is not None:
      doc["edited_time"] = self.edited_time
    if self.is_private is not None:
      doc["is_private"] = self.is_private
    if self.reply_to is not None:
      doc["reply_to"] = self.reply_to
    return doc
