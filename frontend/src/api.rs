use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateTodo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

const API_BASE: &str = "/api";

pub async fn fetch_todos() -> Result<Vec<Todo>, String> {
    Request::get(&format!("{API_BASE}/todos"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Todo>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_todo(title: &str) -> Result<Todo, String> {
    Request::post(&format!("{API_BASE}/todos"))
        .json(&CreateTodo {
            title: title.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Todo>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_todo(id: i64, update: &UpdateTodo) -> Result<Todo, String> {
    Request::patch(&format!("{API_BASE}/todos/{id}"))
        .json(update)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Todo>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_todo(id: i64) -> Result<(), String> {
    let resp = Request::delete(&format!("{API_BASE}/todos/{id}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Delete failed with status {}", resp.status()))
    }
}

pub async fn delete_completed() -> Result<(), String> {
    let resp = Request::delete(&format!("{API_BASE}/todos/completed"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!(
            "Delete completed failed with status {}",
            resp.status()
        ))
    }
}

pub async fn toggle_all() -> Result<Vec<Todo>, String> {
    Request::patch(&format!("{API_BASE}/todos/toggle-all"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Todo>>()
        .await
        .map_err(|e| e.to_string())
}
