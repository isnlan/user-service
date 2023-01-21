// use actix_web::web::{self, ServiceConfig};

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::service::Manager;

mod user;

pub fn route(mgr: Arc<Manager>) -> Router {
    Router::new()
        .route("/api/v1/users", get(user::get_user_list))
        .route("/api/v1/users/create", post(user::create_user))
        .with_state(mgr)
}
