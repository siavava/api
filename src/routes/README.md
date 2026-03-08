# Routes

HTTP endpoint definitions and request handling. Each
module registers its endpoints under a path prefix via a
`register(cfg)` function, composed together in
[`mod.rs`](mod.rs).

## Modules

### [`comments.rs`](comments.rs)

WebSocket endpoint for real-time comment operations.

| Method | Path         | Protocol  | Description      |
|--------|--------------|-----------|------------------|
| GET    | `/comments/` | WebSocket | Comment CRUD.    |

Each incoming text frame is parsed as a `CommentRequest`
(JSON with an `"action"` tag), dispatched to the matching
controller, and a `CommentResponse` is sent back.

### [`location.rs`](location.rs)

REST endpoint for location tracking.

| Method | Path         | Description                    |
|--------|--------------|--------------------------------|
| GET    | `/location/` | Read or update via query params|

With both `city` and `state` query params: records the
new location and returns the previous one. Without both:
returns the current last-known location.

### [`quotes.rs`](quotes.rs)

REST and HTML endpoints for quote display and retrieval.

| Method | Path           | Response | Description        |
|--------|----------------|----------|--------------------|
| GET    | `/`            | HTML     | Quote cycling page |
| GET    | `/one`         | JSON     | Health-check stub  |
| GET    | `/quotes/`     | JSON     | All quotes (array) |
| GET    | `/quotes/test` | Text     | Health-check text  |

### [`views.rs`](views.rs)

REST and SSE endpoints for page view counts.

| Method | Path                 | Description            |
|--------|----------------------|------------------------|
| GET    | `/views/`            | Read view counts       |
| POST   | `/views/`            | Upsert view records    |
| DELETE | `/views/`            | Delete a view document |
| GET    | `/views/watch/`      | SSE live updates       |
| GET    | `/views/watch/test/` | SSE test page (HTML)   |
