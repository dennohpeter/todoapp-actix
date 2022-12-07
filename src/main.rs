use std::env;

use actix_web::{web, App, HttpServer};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod config;
mod db;
mod handlers;
mod models;

use crate::handlers::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    println!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
