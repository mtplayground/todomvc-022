use sqlx::SqlitePool;

use crate::model::{CreateTodo, Todo, UpdateTodo};

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>("SELECT id, title, completed, created_at FROM todos ORDER BY id")
        .fetch_all(pool)
        .await
}

pub async fn create(pool: &SqlitePool, input: &CreateTodo) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES (?) RETURNING id, title, completed, created_at",
    )
    .bind(&input.title)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &SqlitePool,
    id: i64,
    input: &UpdateTodo,
) -> Result<Option<Todo>, sqlx::Error> {
    let existing = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed, created_at FROM todos WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let existing = match existing {
        Some(t) => t,
        None => return Ok(None),
    };

    let title = input.title.as_deref().unwrap_or(&existing.title);
    let completed = input.completed.unwrap_or(existing.completed);

    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = ?, completed = ? WHERE id = ? RETURNING id, title, completed, created_at",
    )
    .bind(title)
    .bind(completed)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(Some(todo))
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_completed(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM todos WHERE completed = TRUE")
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn toggle_all(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let all_completed = sqlx::query_scalar::<_, bool>(
        "SELECT CASE WHEN COUNT(*) = SUM(CASE WHEN completed THEN 1 ELSE 0 END) THEN TRUE ELSE FALSE END FROM todos",
    )
    .fetch_one(pool)
    .await
    .unwrap_or(false);

    let new_state = !all_completed;

    sqlx::query("UPDATE todos SET completed = ?")
        .bind(new_state)
        .execute(pool)
        .await?;

    get_all(pool).await
}
