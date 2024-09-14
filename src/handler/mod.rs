use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "this is to check the health";

    let json_message = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_message)
}
