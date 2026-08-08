#!/usr/bin/env python3
# -*- encoding utf8 -*-

"""
Migration: Normalize `location_history` state fields to the display rule
"country code everywhere, region code only for the US".

Historic records were written as `region_code || country_code`, which yields
opaque values for most of the world (e.g. "Nairobi, 30" — Kenya's Nairobi
County code). The corrected rule is:

  * US visitors      -> state = region code   ("Seattle, WA")
  * everyone else    -> state = country code  ("Nairobi, KE")

Display preference: "UK" instead of ISO-3166's "GB".

The country was never stored, so each document's true country is recovered by
offline reverse-geocoding its `lat`/`lon` (via `reverse_geocoder`). Documents
without coordinates fall back to a curated city -> country table (the town's
canonical top match); beyond that they are left untouched when their state is
already a valid ISO-3166 country code or USPS state code, and reported for
manual follow-up otherwise.

Records written before site namespacing existed all belong to the blog, so any
document without a `namespace` is additionally moved under the blog namespace
("<b>").

Documents missing `lat`/`lon` get coordinates backfilled from the gazetteer
bundled with `reverse_geocoder` (geonames cities > 1000 pop), matched by city
name within the resolved country. US lookups are narrowed by state; ambiguous
US names that cannot be narrowed are reported rather than guessed.

Because `location_history` is keyed by (namespace, city, state), a rewrite can
collide with an existing document (e.g. "Nairobi, 30" -> an existing
"Nairobi, KE"). Colliding documents are merged: counts sum, the latest
timestamp wins, and the freshest coordinates are kept.

The singleton `location` document ("last known location") is rewritten too
when its city+state matches a migrated pair.

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
import csv
import os
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any

import pycountry
import reverse_geocoder as rg
from dotenv import load_dotenv
from pymongo import MongoClient

HISTORY_COLL = "location_history"
LAST_LOCATION_COLL = "location"

# Pre-namespacing records were all blog traffic.
BLOG_NS = "<b>"

# Display-preferred spellings of ISO-3166 alpha-2 codes.
COUNTRY_DISPLAY = {"GB": "UK"}

# Curated fallback for coordinate-less records: the country of the town's
# canonical (most prominent) match. Consulted only when reverse-geocoding
# is impossible. Includes towns whose stored region code coincidentally
# collides with a valid USPS/ISO code (e.g. "Beijing, BJ" — Benin's code —
# or "Haikou, HI", which reads as Hawaii) and would otherwise be kept wrong.
CITY_COUNTRY_OVERRIDES: dict[str, str] = {
  # Kenya
  "Nairobi": "KE", "Kenol": "KE", "Nanyuki": "KE", "Imara Daima Estate": "KE",
  # United Kingdom
  "Hounslow": "UK", "Liverpool": "UK", "Hendon": "UK",
  # France
  "Aulnay-sous-Bois": "FR", "Lestrem": "FR", "Istres": "FR",
  "Gravelines": "FR", "Roubaix": "FR", "Calais": "FR",
  "Corquilleroy": "FR", "Lyon": "FR",
  # Canada
  "Montreal": "CA", "Beauharnois": "CA", "Cochrane": "CA",
  # China
  "Hangzhou": "CN", "Jinhua": "CN", "Shenyang": "CN", "Beijing": "CN",
  "Shanghai": "CN", "Guangzhou": "CN", "Qingdao": "CN", "Haikou": "CN",
  "Putian": "CN", "Quanzhou": "CN",
  # Germany
  "Frankfurt am Main": "DE", "Nuremberg": "DE", "Neunkirchen am Brand": "DE",
  "Hassfurt": "DE", "Falkenstein": "DE", "Brandenburg": "DE",
  # Netherlands
  "Amsterdam": "NL", "Groningen": "NL",
  # Spain
  "Felanitx": "ES", "Chamartin": "ES", "Elda": "ES",
  # Sweden
  "Stockholm": "SE", "Sundbyberg": "SE", "Luleå": "SE",
  # Italy
  "Rome": "IT", "Novate Milanese": "IT",
  # India
  "Surat": "IN", "Markapur": "IN", "Coimbatore": "IN", "Khadki": "IN",
  # Iraq
  "Baghdad": "IQ", "Erbil": "IQ",
  # One-offs
  "Accra": "GH", "Addis Ababa": "ET", "Anyang-si": "KR", "Baku": "AZ",
  "Belgrade": "RS", "Bucharest": "RO", "Clonee": "IE", "Eskişehir": "TR",
  "Ibadan": "NG", "Johannesburg": "ZA", "Kathmandu": "NP",
  "Khartoum": "SD", "Kuala Lumpur": "MY", "Lusaka": "ZM", "Odense": "DK",
  "Provident Centre": "HK",
}

# geonames admin1 name -> USPS code, for US reverse-geocode results.
US_STATES: dict[str, str] = {
  "Alabama": "AL", "Alaska": "AK", "Arizona": "AZ", "Arkansas": "AR",
  "California": "CA", "Colorado": "CO", "Connecticut": "CT", "Delaware": "DE",
  "Florida": "FL", "Georgia": "GA", "Hawaii": "HI", "Idaho": "ID",
  "Illinois": "IL", "Indiana": "IN", "Iowa": "IA", "Kansas": "KS",
  "Kentucky": "KY", "Louisiana": "LA", "Maine": "ME", "Maryland": "MD",
  "Massachusetts": "MA", "Michigan": "MI", "Minnesota": "MN",
  "Mississippi": "MS", "Missouri": "MO", "Montana": "MT", "Nebraska": "NE",
  "Nevada": "NV", "New Hampshire": "NH", "New Jersey": "NJ",
  "New Mexico": "NM", "New York": "NY", "North Carolina": "NC",
  "North Dakota": "ND", "Ohio": "OH", "Oklahoma": "OK", "Oregon": "OR",
  "Pennsylvania": "PA", "Rhode Island": "RI", "South Carolina": "SC",
  "South Dakota": "SD", "Tennessee": "TN", "Texas": "TX", "Utah": "UT",
  "Vermont": "VT", "Virginia": "VA", "Washington": "WA",
  "West Virginia": "WV", "Wisconsin": "WI", "Wyoming": "WY",
  "Washington, D.C.": "DC", "District of Columbia": "DC",
  "Puerto Rico": "PR", "Guam": "GU", "Virgin Islands": "VI",
  "American Samoa": "AS", "Northern Mariana Islands": "MP",
}
USPS_CODES = set(US_STATES.values())

# USPS code -> the geonames admin1 spellings it may appear under.
US_STATE_NAMES: dict[str, set[str]] = {}
for _name, _code in US_STATES.items():
  US_STATE_NAMES.setdefault(_code, set()).add(_name)

# Display code -> ISO code, for gazetteer lookups ("UK" -> "GB").
DISPLAY_COUNTRY = {v: k for k, v in COUNTRY_DISPLAY.items()}

# Manual coordinates for cities the gazetteer misses (neighborhoods, small
# towns, name variants) or where its first match is the wrong namesake
# (e.g. "Haikou" must be Hainan's capital, not the Jiangxi town).
CITY_COORDS_OVERRIDES: dict[str, tuple[float, float]] = {
  "Brandenburg": (52.41667, 12.55),        # Brandenburg an der Havel, DE
  "Clonee": (53.41417, -6.44306),          # Co. Meath, IE
  "Eskişehir": (39.77667, 30.52056),
  "Frankfurt": (50.11552, 8.68417),        # dev-stub spelling of Frankfurt am Main
  "Haikou": (20.04583, 110.34167),         # Hainan capital, not Jiangxi namesake
  "Hyde Park": (42.25538, -71.12533),      # Boston neighborhood, MA
  "Imara Daima Estate": (-1.3234, 36.8541),  # Nairobi estate, KE
  "Jinhua": (29.10678, 119.64421),         # Zhejiang, not Yunnan namesake
  "Kenol": (-0.8996, 37.1305),             # Murang'a county, KE
  "Luleå": (65.58415, 22.15465),
  "New York": (40.71427, -74.00597),
  "Nuremberg": (49.45421, 11.07752),
  "Provident Centre": (22.29083, 114.20028),  # North Point, HK
  "Quanzhou": (24.91389, 118.58583),       # Fujian, not Guangxi namesake
  "Roxbury Crossing": (42.33143, -71.0956),  # Boston neighborhood, MA
  "Washington": (38.90719, -77.03687),     # the District, DC
}


@dataclass
class HistoryDoc:
  """One `location_history` document, as loaded for migration."""

  id: Any
  city: str
  state: str
  namespace: str | None
  count: int
  timestamp: Any
  lat: float | None
  lon: float | None

  @classmethod
  def from_doc(cls, doc: dict[str, Any]) -> "HistoryDoc":
    """Builds a :class:`HistoryDoc` from a raw BSON document."""
    return cls(
      id=doc["_id"],
      city=str(doc.get("city", "")),
      state=str(doc.get("state", "")),
      namespace=doc.get("namespace"),
      count=int(doc.get("count", 0)),
      timestamp=doc.get("timestamp"),
      lat=doc.get("lat"),
      lon=doc.get("lon"),
    )

  @property
  def key(self) -> tuple[str | None, str, str]:
    """The (namespace, city, state) identity the collection is keyed by."""
    return (self.namespace, self.city, self.state)


def is_country_code(state: str) -> bool:
  """Whether ``state`` is a valid ISO-3166 alpha-2 country code."""
  return pycountry.countries.get(alpha_2=state) is not None


def desired_state(
  doc: HistoryDoc, geocoded: dict[Any, Any] | None,
) -> tuple[str | None, str | None, str]:
  """Computes the corrected ``state`` for a history document.

  Args:
    doc: The document under consideration.
    geocoded: The ``reverse_geocoder`` result for the document's
        coordinates, or ``None`` when the document has none.

  Returns:
    ``(new_state, lookup_cc, reason)``. ``new_state`` is ``None`` when the
    document cannot be resolved automatically; ``lookup_cc`` is the ISO
    country code for gazetteer lookups (``"US"`` for US records); ``reason``
    explains the outcome.
  """
  if geocoded is not None:
    cc = geocoded.get("cc", "")
    if not cc:
      return None, None, "reverse geocode returned no country"
    if cc == "US":
      admin1 = geocoded.get("admin1", "")
      code = US_STATES.get(admin1)
      if code is None:
        return None, None, f"unmapped US region {admin1!r}"
      return code, "US", "reverse-geocoded (US region)"
    return COUNTRY_DISPLAY.get(cc, cc), cc, "reverse-geocoded (country)"

  override = CITY_COUNTRY_OVERRIDES.get(doc.city)
  if override is not None:
    return override, DISPLAY_COUNTRY.get(override, override), "curated city override"
  if doc.state in COUNTRY_DISPLAY:
    return COUNTRY_DISPLAY[doc.state], doc.state, "display-preferred country code"
  if doc.state in USPS_CODES:
    return doc.state, "US", "no coordinates; state already a valid code"
  if doc.state in COUNTRY_DISPLAY.values():
    return doc.state, DISPLAY_COUNTRY[doc.state], "no coordinates; state already a valid code"
  if is_country_code(doc.state):
    return doc.state, doc.state, "no coordinates; state already a valid code"
  return None, None, "no coordinates and unrecognizable state"


def load_city_index() -> dict[tuple[str, str], list[tuple[float, float, str]]]:
  """Indexes ``reverse_geocoder``'s bundled gazetteer by (city, country).

  Returns:
    ``{(name_casefold, cc): [(lat, lon, admin1), ...]}`` over geonames'
    cities-with-population-over-1000 dataset.
  """
  path = Path(rg.__file__).resolve().parent / "rg_cities1000.csv"
  index: dict[tuple[str, str], list[tuple[float, float, str]]] = {}
  with open(path, newline="", encoding="utf-8") as fh:
    for row in csv.DictReader(fh):
      key = (row["name"].casefold(), row["cc"])
      index.setdefault(key, []).append(
        (float(row["lat"]), float(row["lon"]), row["admin1"]),
      )
  return index


def lookup_coords(
  index: dict[tuple[str, str], list[tuple[float, float, str]]],
  city: str,
  cc: str | None,
  state_code: str | None = None,
) -> tuple[float, float] | None:
  """Finds coordinates for a city by name within a country.

  Manual overrides win. US lookups are narrowed by state; an ambiguous US
  name that cannot be narrowed returns ``None`` rather than guessing a
  namesake in the wrong state. Elsewhere the gazetteer's top match for the
  country is taken.
  """
  manual = CITY_COORDS_OVERRIDES.get(city)
  if manual is not None:
    return manual
  if cc is None:
    return None
  candidates = index.get((city.casefold(), cc), [])
  if not candidates:
    return None
  if cc == "US" and state_code:
    names = US_STATE_NAMES.get(state_code, set())
    narrowed = [c for c in candidates if c[2] in names]
    if narrowed:
      candidates = narrowed
    elif len(candidates) > 1:
      return None
  return candidates[0][0], candidates[0][1]


def migrate(db_name: str, mongo_uri: str, *, apply: bool = False) -> None:
  """Rewrites non-US ``state`` fields to country codes, merging collisions.

  Loads every history document, recovers each document's true country from
  its coordinates (falling back to the curated city table), moves
  pre-namespacing documents under the blog namespace, plans the rewrites
  (grouping documents that converge on the same (namespace, city, state)
  identity), prints the plan, and optionally applies it.

  Args:
    db_name: MongoDB database name (e.g. ``feed`` or ``feed-dev``).
    mongo_uri: MongoDB connection string.
    apply: If ``False`` (default), perform a dry run. If ``True``, write
        the updates to the database.
  """
  client: MongoClient[dict[str, Any]] = MongoClient(mongo_uri)
  db = client[db_name]
  history = db[HISTORY_COLL]

  docs = [HistoryDoc.from_doc(d) for d in history.find({})]
  if not docs:
    print("No documents in location_history.")
    return

  with_coords = [d for d in docs if d.lat is not None and d.lon is not None]
  geocoded: dict[Any, dict[Any, Any]] = {}
  if with_coords:
    results = rg.search([(d.lat, d.lon) for d in with_coords], mode=1)
    geocoded = {d.id: r for d, r in zip(with_coords, results)}

  groups: dict[tuple[str, str, str], list[HistoryDoc]] = {}
  unresolved: list[tuple[HistoryDoc, str]] = []
  unchanged_by_key: dict[tuple[str | None, str, str], HistoryDoc] = {}

  lookup_cc_by_id: dict[Any, str | None] = {}
  for doc in docs:
    new_state, lookup_cc, reason = desired_state(doc, geocoded.get(doc.id))
    lookup_cc_by_id[doc.id] = lookup_cc
    if new_state is None:
      unresolved.append((doc, reason))
      continue
    new_ns = doc.namespace or BLOG_NS
    if new_state == doc.state and new_ns == doc.namespace:
      unchanged_by_key[doc.key] = doc
    else:
      groups.setdefault((new_ns, doc.city, new_state), []).append(doc)

  city_index = load_city_index()
  coord_missing: list[tuple[str | None, str, str, int]] = []

  if not groups:
    print("No documents need migration.")
  else:
    total = sum(len(sources) for sources in groups.values())
    print(f"Found {total} document(s) to migrate into {len(groups)} identit(ies):\n")

  for (namespace, city, new_state), sources in sorted(
    groups.items(), key=lambda item: (item[0][0] or "", item[0][1]),
  ):
    existing = unchanged_by_key.get((namespace, city, new_state))
    merged_count = sum(s.count for s in sources) + (existing.count if existing else 0)
    candidates = sources + ([existing] if existing else [])
    ts = lambda d: d.timestamp or datetime.min  # noqa: E731
    latest = max(candidates, key=ts)
    freshest_coords = max(
      (d for d in candidates if d.lat is not None and d.lon is not None),
      key=ts,
      default=None,
    )

    filled_coords: tuple[float, float] | None = None
    if freshest_coords is None:
      cc = lookup_cc_by_id[sources[0].id]
      filled_coords = lookup_coords(city_index, city, cc, new_state if cc == "US" else None)
      if filled_coords is None:
        coord_missing.append((namespace, city, new_state, merged_count))

    for source in sources:
      src_label = f"[{source.namespace}]" if source.namespace else "[—]"
      print(f"  {src_label} {source.city!r}, {source.state!r}  ->  [{namespace}] {city!r}, {new_state!r}  (count {source.count})")
    if existing:
      print(f"    merging into existing [{namespace}] {city!r}, {new_state!r}  (count {existing.count})")
    coords_note = f", coords backfilled {filled_coords}" if filled_coords else ""
    print(f"    => count {merged_count}, timestamp {latest.timestamp}{coords_note}")

    if not apply:
      continue

    survivor = existing or max(sources, key=lambda d: d.count)
    update: dict[str, Any] = {"state": new_state, "namespace": namespace, "count": merged_count}
    if latest.timestamp is not None:
      update["timestamp"] = latest.timestamp
    if freshest_coords is not None:
      update["lat"] = freshest_coords.lat
      update["lon"] = freshest_coords.lon
    elif filled_coords is not None:
      update["lat"], update["lon"] = filled_coords
    history.update_one({"_id": survivor.id}, {"$set": update})
    stale_ids = [d.id for d in sources if d.id != survivor.id]
    if stale_ids:
      history.delete_many({"_id": {"$in": stale_ids}})

    db[LAST_LOCATION_COLL].update_many(
      {"city": city, "state": {"$in": [s.state for s in sources]}},
      {"$set": {"state": new_state}},
    )

  coord_fills: list[tuple[HistoryDoc, float, float]] = []
  for doc in unchanged_by_key.values():
    if doc.lat is not None and doc.lon is not None:
      continue
    cc = lookup_cc_by_id[doc.id]
    found = lookup_coords(city_index, doc.city, cc, doc.state if cc == "US" else None)
    if found is None:
      coord_missing.append((doc.namespace, doc.city, doc.state, doc.count))
    else:
      coord_fills.append((doc, found[0], found[1]))

  if coord_fills:
    print(f"\n{len(coord_fills)} untouched document(s) get backfilled coordinates:")
    for doc, lat, lon in coord_fills:
      print(f"  [{doc.namespace}] {doc.city!r}, {doc.state!r}  ->  ({lat}, {lon})")
      if apply:
        history.update_one({"_id": doc.id}, {"$set": {"lat": lat, "lon": lon}})

  if coord_missing:
    print(f"\n{len(coord_missing)} document(s) still lack coordinates (no gazetteer match):")
    for ns, city, state, count in coord_missing:
      print(f"  [{ns or '—'}] {city!r}, {state!r}  (count {count})")

  if unresolved:
    print(f"\n{len(unresolved)} document(s) could not be resolved automatically:")
    for doc, reason in unresolved:
      ns_label = f"[{doc.namespace}] " if doc.namespace else ""
      print(f"  {ns_label}{doc.city!r}, {doc.state!r}  (count {doc.count})  — {reason}")

  if not apply:
    print("\nDry run — no changes written. Re-run with --apply to commit.")
  elif groups:
    print("\nMigration applied.")


def main() -> None:
  """CLI entry point. Parse arguments, load ``.env``, and run the migration."""
  parser = argparse.ArgumentParser(description="Normalize location_history state codes.")
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
  mongo_uri: str = args.uri or os.environ.get("MONGODB_URI", "mongodb://localhost:27017")
  apply: bool = args.apply

  print(f"Database: {db_name}")
  print(f"URI:      {mongo_uri}")
  print(f"Mode:     {'APPLY' if apply else 'DRY RUN'}\n")

  migrate(db_name, mongo_uri, apply=apply)


if __name__ == "__main__":
  main()
