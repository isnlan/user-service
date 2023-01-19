use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use handler::Controller;

mod handler;
mod model;
mod service;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let ctrl = Arc::new(Controller {
        UserSerice: service::UserService::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ctrl.clone()))
            .configure(handler::app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
