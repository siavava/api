#!/user/bin/env python3
# -*- encoding utf8 -*-

"""
Migration: Rewrite comment `path` fields from "/<path>" to "<b>:/<path>".

Usage:
  # Dry run (default) — prints what would change, writes nothing.
  python migrate.py

  # Live run — applies the updates to MongoDB.
  python migrate.py --apply

Environment variables:
  MONGODB_URI — MongoDB connection string (default: mongodb://localhost:27017)
  DB_NAME    — Database name            (default: feed-dev)
"""

from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path

from dotenv import load_dotenv
from pymongo import MongoClient

# Allow imports from the migrations package root.
_migrations_root = str(Path(__file__).resolve().parents[1])
if _migrations_root not in sys.path:
  sys.path.insert(0, _migrations_root)

from models.comment import Comment  # noqa: E402

PREFIX = "<b>:"


def migrate(db_name: str, mongo_uri: str, *, apply: bool = False) -> None:
  """Prefix comment ``path`` fields with :data:`PREFIX` (``<b>:``).

  Finds all comments whose ``path`` starts with ``/`` but is not already
  prefixed, prints the planned changes, and optionally applies them via a
  single ``update_many`` with an aggregation pipeline.

  Args:
    db_name: MongoDB database name (e.g. ``feed`` or ``feed-dev``).
    mongo_uri: MongoDB connection string.
    apply: If ``False`` (default), perform a dry run. If ``True``, write
        the updates to the database.
  """
  client: MongoClient[dict[str, object]] = MongoClient(mongo_uri)
  db = client[db_name]
  collection = db["comments"]

  # Find comments whose path starts with "/" but NOT already prefixed.
  query: dict[str, object] = {"path": {"$regex": r"^/", "$not": {"$regex": r"^<b>:"}}}
  cursor = collection.find(query)

  updates: list[tuple[Comment, str]] = []
  for doc in cursor:
    comment = Comment.from_doc(doc)
    new_path = f"{PREFIX}{comment.path}"
    updates.append((comment, new_path))

  if not updates:
    print("No documents need migration.")
    return

  print(f"Found {len(updates)} document(s) to update:\n")
  for comment, new_path in updates:
    print(f"  {comment.id}:  {comment.path!r}  ->  {new_path!r}")

  if not apply:
    print(f"\nDry run — no changes written. Re-run with --apply to commit.")
    return

  result = collection.update_many(query, [
    {"$set": {"path": {"$concat": [PREFIX, "$path"]}}},
  ])

  print(f"\nUpdated {result.modified_count} document(s).")


def main() -> None:
  """CLI entry point. Parse arguments, load ``.env``, and run the migration."""
  parser = argparse.ArgumentParser(description="Migrate comment paths.")
  parser.add_argument(
    "--apply",
    action="store_true",
    help="Apply changes (default is dry run).",
  )
  parser.add_argument(
    "--db",
    default=None,
    help="Database name (default: $DB_NAME or feed-dev).",
  )
  parser.add_argument(
    "--uri",
    default=None,
    help="MongoDB URI (default: $MONGODB_URI from .env or mongodb://localhost:27017).",
  )
  args = parser.parse_args()

  # Load .env from project root (two levels up from this script).
  env_path = Path(__file__).resolve().parents[2] / ".env"
  load_dotenv(env_path)

  db_name: str = args.db or os.environ.get("DB_NAME", "feed-dev")
  mongo_uri: str = args.uri or os.environ.get("MONGODB_URI", "mongodb://localhost:27017")
  apply: bool = args.apply

  print(f"Database: {db_name}")
  print(f"URI:      {mongo_uri}")
  print(f"Mode:     {'APPLY' if apply else 'DRY RUN'}\n")

  migrate(db_name, mongo_uri, apply=apply)


if __name__ == "__main__":
  main()
