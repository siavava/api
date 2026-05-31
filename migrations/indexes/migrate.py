#!/usr/bin/env python3
# -*- encoding utf8 -*-

"""
Migration: Create the indexes the Rust API filters and sorts on.

Idempotent — `create_index` is a no-op if an equivalent index already
exists. Safe to re-run.

Usage:
  # Dry run (default) — prints what would be created, writes nothing.
  python migrate.py

  # Live run — creates indexes on MongoDB.
  python migrate.py --apply

Environment variables:
  MONGODB_URI — MongoDB connection string (default: mongodb://localhost:27017)
  DB_NAME     — Database name             (default: feed-dev)
"""

from __future__ import annotations

import argparse
import os
from dataclasses import dataclass
from pathlib import Path

from dotenv import load_dotenv
from pymongo import ASCENDING, DESCENDING, MongoClient


@dataclass(frozen=True)
class IndexSpec:
  """One index to ensure on a Mongo collection."""

  collection: str
  name: str
  keys: list[tuple[str, int]]
  rationale: str
  unique: bool = False
  # MongoDB TTL: documents expire `expire_after_seconds` past the indexed
  # field's value. Only valid on a single-field index over a date column.
  expire_after_seconds: int | None = None


INDEXES: list[IndexSpec] = [
  IndexSpec(
    collection="comments",
    name="path_1",
    keys=[("path", ASCENDING)],
    rationale="list_comments filters by path",
  ),
  IndexSpec(
    collection="views",
    name="route_1",
    keys=[("route", ASCENDING)],
    unique=True,
    rationale="get_views/insert_view upsert key (one doc per route)",
  ),
  IndexSpec(
    collection="views",
    name="count_-1",
    keys=[("count", DESCENDING)],
    rationale="get_all_views sorts by count desc",
  ),
  IndexSpec(
    collection="location_history",
    name="city_1_state_1",
    keys=[("city", ASCENDING), ("state", ASCENDING)],
    rationale="update_location_history upsert filter",
  ),
  IndexSpec(
    collection="now",
    name="key_1",
    keys=[("key", ASCENDING)],
    unique=True,
    rationale="one document per slot key — set_now upsert filter",
  ),
  IndexSpec(
    collection="now",
    name="expires_at_1",
    keys=[("expires_at", ASCENDING)],
    expire_after_seconds=0,
    rationale="TTL — Mongo sweeps documents at expires_at",
  ),
]


def migrate(db_name: str, mongo_uri: str, *, apply: bool = False) -> None:
  """Create missing indexes on the configured MongoDB database.

  Args:
    db_name: MongoDB database name (e.g. ``feed`` or ``feed-dev``).
    mongo_uri: MongoDB connection string.
    apply: If ``False`` (default), perform a dry run. If ``True``, create
        the indexes on the database.
  """
  client: MongoClient[dict[str, object]] = MongoClient(mongo_uri)
  db = client[db_name]

  print(f"Planned indexes ({len(INDEXES)}):\n")
  for spec in INDEXES:
    key_repr = ", ".join(
      f"{k}:{'asc' if v == ASCENDING else 'desc'}" for k, v in spec.keys
    )
    flags = ""
    if spec.unique:
      flags += " unique"
    if spec.expire_after_seconds is not None:
      flags += f" ttl={spec.expire_after_seconds}s"
    print(f"  {spec.collection}.{spec.name}  [{key_repr}]{flags}")
    print(f"    why: {spec.rationale}")

  if not apply:
    print("\nDry run — no indexes created. Re-run with --apply to commit.")
    return

  print()
  for spec in INDEXES:
    kwargs: dict[str, object] = {"name": spec.name, "unique": spec.unique}
    if spec.expire_after_seconds is not None:
      kwargs["expireAfterSeconds"] = spec.expire_after_seconds
    result = db[spec.collection].create_index(spec.keys, **kwargs)
    print(f"  ensured {spec.collection}.{spec.name} -> {result}")

  print("\nDone.")


def main() -> None:
  """CLI entry point. Parse arguments, load ``.env``, and run the migration."""
  parser = argparse.ArgumentParser(description="Create Mongo indexes.")
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

  env_path = Path(__file__).resolve().parents[2] / ".env"
  load_dotenv(env_path)

  db_name: str = args.db or os.environ.get("DB_NAME", "feed-dev")
  mongo_uri: str = args.uri or os.environ.get(
    "MONGODB_URI", "mongodb://localhost:27017"
  )
  apply: bool = args.apply

  print(f"Database: {db_name}")
  print(f"URI:      {mongo_uri}")
  print(f"Mode:     {'APPLY' if apply else 'DRY RUN'}\n")

  migrate(db_name, mongo_uri, apply=apply)


if __name__ == "__main__":
  main()
