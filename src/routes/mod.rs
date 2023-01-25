// use actix_web::web::{self, ServiceConfig};

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::service::Manager;

mod sse;
mod user;

pub fn route(mgr: Arc<Manager>) -> Router {
    Router::new()
        .route("/api/v1/users", get(user::get_user_list))
        .route("/api/v1/users/create", post(user::create_user))
        .route("/api/v1/sse", get(sse::sse_handler))
        .fallback(handler_404)
        .with_state(mgr)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
