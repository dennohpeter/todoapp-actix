use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

use crate::db;
use crate::models::Status;

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

pub async fn get_todos(pool: web::Data<Pool>) -> impl Responder {
    let client: Client = pool.get().await.expect("Error connecting to the database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
