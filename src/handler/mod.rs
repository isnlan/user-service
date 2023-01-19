use actix_web::web::{self, ServiceConfig};

use crate::service;

mod user;

pub fn app_config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .route("/users", web::get().to(user::get_user_list))
            .route("/users/create", web::post().to(user::create_user)),
    );
}

pub struct Controller {
    pub UserSerice: service::UserService,
}
