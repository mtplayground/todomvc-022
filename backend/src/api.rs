use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::SqlitePool;

use crate::db;
use crate::model::CreateTodo;

pub async fn list_todos(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match db::get_all(&pool).await {
        Ok(todos) => Json(todos).into_response(),
        Err(e) => {
            tracing::error!("Failed to list todos: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    if input.title.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match db::create(&pool, &input).await {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create todo: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
