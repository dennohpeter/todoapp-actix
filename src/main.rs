use actix_web::{App, HttpResponse, HttpServer, Responder};

mod models;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(models::Status {
        status: "Ok".to_string(),
    })
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| App::new().route("/", actix_web::web::get().to(status)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
