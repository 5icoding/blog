// fn main() {
//     println!("Hello, world!");
// }

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello Actix_web!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(greet)
        .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}