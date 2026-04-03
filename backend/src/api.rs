use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::SqlitePool;

use crate::db;
use crate::model::{CreateTodo, UpdateTodo};

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

pub async fn update_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTodo>,
) -> impl IntoResponse {
    match db::update(&pool, id, &input).await {
        Ok(Some(todo)) => Json(todo).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to update todo {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match db::delete(&pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to delete todo {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_completed(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match db::delete_completed(&pool).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Failed to delete completed todos: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn toggle_all(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match db::toggle_all(&pool).await {
        Ok(todos) => Json(todos).into_response(),
        Err(e) => {
            tracing::error!("Failed to toggle all todos: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
