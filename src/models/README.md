# Models

Data models (structs and enums) shared across the API.
These types are used by both
[controllers](../controllers/) (for DB operations) and
[routes](../routes/) (for request/response
serialization).

## Modules

### [`comments.rs`](comments.rs)

Blog comment data models and the WebSocket protocol:

- **`BlogComment`** — Database model stored in the
  `comments` MongoDB collection.
- **`PopulatedComment`** — API-facing comment with its
  reply tree fully resolved (nested
  `PopulatedComment`s instead of flat ID strings).
- **`CommentEdit`** — Partial update payload for
  editing a comment's text.
- **`CommentRequest`** — Incoming WebSocket message,
  discriminated by an `"action"` tag. Variants:
  `Create`, `Edit`, `Like`, `Delete`, `List`.
- **`CommentResponse`** — Outgoing WebSocket message,
  discriminated by a `"type"` tag. Variants:
  `Created`, `Updated`, `Liked`, `Deleted`, `List`,
  `Error`.

### [`location.rs`](location.rs)

Location tracking data:

- **`LocationData`** — Simple city + state struct with
  `ByteString` conversions for SSE transmission.

### [`views.rs`](views.rs)

Page view count tracking:

- **`PageViews`** — Tracks the view count for a single
  page route.
- Custom `PartialEq` implementation based solely on
  `route` (used by `EventsBroadcaster` for SSE filter
  matching).
- `ByteString` conversions for SSE transmission.
