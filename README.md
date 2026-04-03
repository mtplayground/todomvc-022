# TodoMVC — Leptos + Axum + SQLite

A full-stack [TodoMVC](http://todomvc.com) implementation using Rust, with a Leptos CSR frontend compiled to WebAssembly and an Axum backend with SQLite persistence.

## Architecture

```
frontend/     Leptos CSR app (compiled to WASM via Trunk)
backend/      Axum REST API with SQLite (via sqlx)
```

- **Frontend**: Leptos 0.6 with client-side rendering, gloo-net HTTP client, hash-based routing
- **Backend**: Axum 0.7 serving the REST API and static frontend assets
- **Database**: SQLite with sqlx migrations

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) — `cargo install trunk`
- `wasm32-unknown-unknown` target — `rustup target add wasm32-unknown-unknown`

## Setup

```bash
# Clone the repository
git clone https://github.com/mtplayground/todomvc-022.git
cd todomvc-022

# Build the frontend
cd frontend
trunk build --release
cd ..

# Run the backend (serves frontend from frontend/dist/)
cd backend
DIST_DIR=../frontend/dist DATABASE_URL="sqlite:todos.sqlite?mode=rwc" cargo run --release
```

The app will be available at [http://localhost:8080](http://localhost:8080).

## Development

```bash
# Run backend tests
cargo test --package backend

# Run clippy
cargo clippy --all-targets

# Frontend dev server (with hot reload, proxies API to backend)
cd frontend
trunk serve
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/api/todos` | List all todos |
| POST | `/api/todos` | Create a todo |
| PATCH | `/api/todos/:id` | Update a todo |
| DELETE | `/api/todos/:id` | Delete a todo |
| DELETE | `/api/todos/completed` | Clear completed todos |
| PATCH | `/api/todos/toggle-all` | Toggle all todos |

## License

MIT
