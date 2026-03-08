# Controllers

Core logic layer for the API. Each module encapsulates
the domain logic for a specific feature area, operating
directly against MongoDB collections and returning domain
types from [`models`](../models/).

## Modules

### [`comments.rs`](comments.rs)

CRUD operations for blog comments, including:

- **Create** — insert a new comment (optionally as a
  reply to a parent).
- **Edit** — update a comment's text and set
  `edited_time`.
- **Like** — increment a comment's like count.
- **Delete** — recursively remove a comment and all
  nested replies.
- **List** — fetch top-level comments for a page with
  fully populated reply trees.

### [`events.rs`](events.rs)

Generic SSE (Server-Sent Events) broadcaster backed by
MongoDB change streams.

- `EventsBroadcaster<T>` — watches a collection for
  changes and pushes updates to connected SSE clients.
- Includes automatic heartbeat pings to detect and prune
  stale connections.
- Used by the views system for real-time page view
  updates.

### [`location.rs`](location.rs)

Location tracking with two MongoDB collections:

- **`location`** — stores the single most-recent
  location.
- **`location_history`** — one document per unique
  city+state pair, with a visit count and timestamp.

Also exports the `location!` convenience macro.

### [`views.rs`](views.rs)

Page view counting and retrieval:

- **get_views** — fetch a route's view count, optionally
  incrementing it atomically.
- **insert_view / insert_views** — upsert one or more
  view-count records.
- **delete_views** — remove a route's view-count
  document.
- **get_all_views** — fetch all view counts, sorted by
  count descending.

Also exports the `views!` and `all_views!` convenience
macros.

## Re-exports

- `EventsBroadcaster` is re-exported from
  [`events`](events.rs) at the module level.
