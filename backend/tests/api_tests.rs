use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

async fn setup() -> axum::Router {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to connect to in-memory db");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    backend::build_api_router(pool)
}

async fn body_json(body: Body) -> Value {
    let bytes = body.collect().await.expect("failed to read body").to_bytes();
    serde_json::from_slice(&bytes).expect("failed to parse JSON")
}

fn json_request(method: &str, uri: &str, body: Option<Value>) -> Request<Body> {
    let builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");

    match body {
        Some(b) => builder
            .body(Body::from(serde_json::to_string(&b).expect("serialize")))
            .expect("build request"),
        None => builder.body(Body::empty()).expect("build request"),
    }
}

// --- Health ---

#[tokio::test]
async fn test_health() {
    let app = setup().await;
    let req = Request::get("/health").body(Body::empty()).expect("request");
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);
}

// --- Create ---

#[tokio::test]
async fn test_create_todo() {
    let app = setup().await;
    let req = json_request("POST", "/api/todos", Some(json!({"title": "Buy milk"})));
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body = body_json(resp.into_body()).await;
    assert_eq!(body["title"], "Buy milk");
    assert_eq!(body["completed"], false);
    assert!(body["id"].is_number());
}

#[tokio::test]
async fn test_create_todo_empty_title() {
    let app = setup().await;
    let req = json_request("POST", "/api/todos", Some(json!({"title": ""})));
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_todo_whitespace_title() {
    let app = setup().await;
    let req = json_request("POST", "/api/todos", Some(json!({"title": "   "})));
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// --- List ---

#[tokio::test]
async fn test_list_todos_empty() {
    let app = setup().await;
    let req = Request::get("/api/todos").body(Body::empty()).expect("request");
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_json(resp.into_body()).await;
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn test_list_todos_with_items() {
    let app = setup().await;

    // Create two todos
    let req = json_request("POST", "/api/todos", Some(json!({"title": "First"})));
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = json_request("POST", "/api/todos", Some(json!({"title": "Second"})));
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::CREATED);

    // List
    let req = Request::get("/api/todos").body(Body::empty()).expect("request");
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_json(resp.into_body()).await;
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["title"], "First");
    assert_eq!(arr[1]["title"], "Second");
}

// --- Update ---

#[tokio::test]
async fn test_update_todo_title() {
    let app = setup().await;

    let req = json_request("POST", "/api/todos", Some(json!({"title": "Original"})));
    let resp = app.clone().oneshot(req).await.expect("response");
    let created = body_json(resp.into_body()).await;
    let id = created["id"].as_i64().expect("id");

    let req = json_request(
        "PATCH",
        &format!("/api/todos/{}", id),
        Some(json!({"title": "Updated"})),
    );
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_json(resp.into_body()).await;
    assert_eq!(body["title"], "Updated");
    assert_eq!(body["completed"], false);
}

#[tokio::test]
async fn test_update_todo_completed() {
    let app = setup().await;

    let req = json_request("POST", "/api/todos", Some(json!({"title": "Task"})));
    let resp = app.clone().oneshot(req).await.expect("response");
    let created = body_json(resp.into_body()).await;
    let id = created["id"].as_i64().expect("id");

    let req = json_request(
        "PATCH",
        &format!("/api/todos/{}", id),
        Some(json!({"completed": true})),
    );
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_json(resp.into_body()).await;
    assert_eq!(body["title"], "Task");
    assert_eq!(body["completed"], true);
}

#[tokio::test]
async fn test_update_nonexistent_todo() {
    let app = setup().await;
    let req = json_request(
        "PATCH",
        "/api/todos/9999",
        Some(json!({"title": "Nope"})),
    );
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// --- Delete ---

#[tokio::test]
async fn test_delete_todo() {
    let app = setup().await;

    let req = json_request("POST", "/api/todos", Some(json!({"title": "To delete"})));
    let resp = app.clone().oneshot(req).await.expect("response");
    let created = body_json(resp.into_body()).await;
    let id = created["id"].as_i64().expect("id");

    let req = Request::delete(&format!("/api/todos/{}", id))
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let req = Request::get("/api/todos").body(Body::empty()).expect("request");
    let resp = app.oneshot(req).await.expect("response");
    let body = body_json(resp.into_body()).await;
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn test_delete_nonexistent_todo() {
    let app = setup().await;
    let req = Request::delete("/api/todos/9999")
        .body(Body::empty())
        .expect("request");
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// --- Delete Completed ---

#[tokio::test]
async fn test_delete_completed() {
    let app = setup().await;

    // Create two todos
    let req = json_request("POST", "/api/todos", Some(json!({"title": "Keep"})));
    app.clone().oneshot(req).await.expect("response");

    let req = json_request("POST", "/api/todos", Some(json!({"title": "Remove"})));
    let resp = app.clone().oneshot(req).await.expect("response");
    let created = body_json(resp.into_body()).await;
    let id = created["id"].as_i64().expect("id");

    // Mark second as completed
    let req = json_request(
        "PATCH",
        &format!("/api/todos/{}", id),
        Some(json!({"completed": true})),
    );
    app.clone().oneshot(req).await.expect("response");

    // Delete completed
    let req = Request::delete("/api/todos/completed")
        .header("content-type", "application/json")
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify only "Keep" remains
    let req = Request::get("/api/todos").body(Body::empty()).expect("request");
    let resp = app.oneshot(req).await.expect("response");
    let body = body_json(resp.into_body()).await;
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["title"], "Keep");
}

// --- Toggle All ---

#[tokio::test]
async fn test_toggle_all() {
    let app = setup().await;

    // Create two todos (both uncompleted)
    let req = json_request("POST", "/api/todos", Some(json!({"title": "A"})));
    app.clone().oneshot(req).await.expect("response");

    let req = json_request("POST", "/api/todos", Some(json!({"title": "B"})));
    app.clone().oneshot(req).await.expect("response");

    // Toggle all -> should mark all completed
    let req = Request::builder()
        .method("PATCH")
        .uri("/api/todos/toggle-all")
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_json(resp.into_body()).await;
    let arr = body.as_array().expect("array");
    assert!(arr.iter().all(|t| t["completed"] == true));

    // Toggle all again -> should mark all uncompleted
    let req = Request::builder()
        .method("PATCH")
        .uri("/api/todos/toggle-all")
        .body(Body::empty())
        .expect("request");
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_json(resp.into_body()).await;
    let arr = body.as_array().expect("array");
    assert!(arr.iter().all(|t| t["completed"] == false));
}

#[tokio::test]
async fn test_toggle_all_mixed() {
    let app = setup().await;

    // Create two todos
    let req = json_request("POST", "/api/todos", Some(json!({"title": "A"})));
    let resp = app.clone().oneshot(req).await.expect("response");
    let a = body_json(resp.into_body()).await;

    let req = json_request("POST", "/api/todos", Some(json!({"title": "B"})));
    app.clone().oneshot(req).await.expect("response");

    // Mark first as completed
    let req = json_request(
        "PATCH",
        &format!("/api/todos/{}", a["id"].as_i64().expect("id")),
        Some(json!({"completed": true})),
    );
    app.clone().oneshot(req).await.expect("response");

    // Toggle all with mixed state -> should mark all completed
    let req = Request::builder()
        .method("PATCH")
        .uri("/api/todos/toggle-all")
        .body(Body::empty())
        .expect("request");
    let resp = app.oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    let body = body_json(resp.into_body()).await;
    let arr = body.as_array().expect("array");
    assert!(arr.iter().all(|t| t["completed"] == true));
}
