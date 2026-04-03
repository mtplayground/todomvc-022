pub mod api;
pub mod db;
pub mod model;

use axum::routing::{delete, get, patch};
use axum::Router;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

async fn health() -> &'static str {
    "ok"
}

pub fn build_api_router(pool: SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .route("/todos", get(api::list_todos).post(api::create_todo))
        .route("/todos/completed", delete(api::delete_completed))
        .route("/todos/toggle-all", patch(api::toggle_all))
        .route("/todos/:id", patch(api::update_todo).delete(api::delete_todo));

    let dist_dir = std::env::var("DIST_DIR").unwrap_or_else(|_| "dist".into());
    let index_path = format!("{}/index.html", dist_dir);

    Router::new()
        .route("/health", get(health))
        .nest("/api", api)
        .nest_service(
            "/",
            ServeDir::new(&dist_dir).not_found_service(ServeFile::new(&index_path)),
        )
        .layer(cors)
        .with_state(pool)
}
