use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::health_checker_handler, AppStatus};

pub fn create_router(app_status: Arc<AppStatus>) -> Router {
    Router::new().route("/api/healthchecker/", get(health_checker_handler))
}
