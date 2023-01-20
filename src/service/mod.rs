use std::sync::Arc;

mod user_service;
use crate::model::*;

pub struct Manager {
    pub user_serice: user_service::UserService,
}

impl Manager {
    pub fn new(pool: sqlx::PgPool) -> Arc<Self> {
        let mgr = Manager {
            user_serice: user_service::UserService::new(UserOp::new(pool)),
        };

        Arc::new(mgr)
    }
}
