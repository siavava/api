"""Shared data models for migration scripts.

These mirror the Rust structs in ``src/models/`` for use in Python-based
MongoDB migrations.
"""

from .comment import Comment

__all__ = ["Comment"]
