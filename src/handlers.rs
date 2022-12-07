use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use std::io::ErrorKind::Other;

use crate::db;
use crate::models::{CreateTodoList, ResultResponse, Status};

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

pub async fn get_items(pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = pool.get().await.expect("Error connecting to the database");

    let result = db::get_items(&client, path.0).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_todo(pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client = pool.get().await.expect("Error connecting to the database");

    let result = db::create_todo(&client, json.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn check_item(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = pool.get().await.expect("Error connecting to the database");

    let result = db::check_item(&client, path.0, path.1).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse { success: true }),
        Err(ref e) if e.kind() == Other => {
            HttpResponse::Ok().json(ResultResponse { success: false })
        }
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
