# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust API backend (Actix-Web + MongoDB) supporting real-time features: page view counting (SSE), blog comments (WebSocket), location tracking, and quotes. Deployed on Fly.io via Docker.

## Build & Development Commands

```bash
make build          # cargo build --release
make dev            # cargo watch -x run (auto-reload)
make test           # cargo test --offline -- --color=always --nocapture
make style-check    # cargo fmt --all -- --check
make lint           # cargo clippy --all-targets --all-features -- -D warnings
make docs           # cargo doc --no-deps
make clean          # cargo clean
```

CI runs: build, test, clippy (deny warnings), and cargo-audit on push to main / PRs.

## Architecture

**Layered structure:** Routes → Controllers → Models → Database

- `src/main.rs` — Server setup, middleware stack, route registration
- `src/lib.rs` — `AppState` struct, convenience macros (`views!`, `all_views!`, `location!`, `app_state!`)
- `src/db.rs` — MongoDB collection accessor helpers
- `src/models/` — Data structs with serde derive (PageViews, LocationData, BlogComment, CommentRequest/Response)
- `src/controllers/` — Business logic and DB operations (views, comments, location, generic SSE broadcaster)
- `src/routes/` — HTTP/WebSocket handlers (views, comments, location, quotes)

### Key Patterns

- `AppState` holds MongoDB client, SSE broadcaster (`EventsBroadcaster<T>`), and tokio broadcast channel for comment events — passed via `web::Data<AppState>`
- `EventsBroadcaster<T>` is a generic SSE broadcaster backed by MongoDB change streams
- Comments use WebSocket with JSON frames and `#[serde(tag = "action")]` discriminated enums
- Comment mutations broadcast to all clients on the same route via `tokio::sync::broadcast`
- Background tasks: SSE heartbeat ping (3s interval), MongoDB change stream listener

### Database

- Debug builds use `feed-dev`, release builds use `feed` (compile-time switch)
- Collections: `views`, `comments`, `location`, `location_history`, `quotes`
- Required env: `MONGODB_URI`. Optional: `PORT` (default 3000), `RUST_LOG`

## Commit Convention

Prefix with category in parens followed by a colon: `(feat):`, `(fix):`, `(update):`, `(chore):` — lowercase title.
