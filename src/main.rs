use std::env;

use actix_web::{web::Data, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod errors;
mod model;
mod routes;
mod service;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let connection_str = env::var("DATABASE_URL")
        .unwrap_or("postgres://snlan:123456@192.168.36.130:5432/postgres".to_string());
    // .expect("数据库连接字符串获取失败，请检查env文件是否已配置数据库连接字符串");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str)
        .await
        .unwrap();

    let mgr = service::Manager::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(mgr.clone()))
            .configure(routes::app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
