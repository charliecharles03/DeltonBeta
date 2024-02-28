use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use controller::{health_check, get_Brag};
mod controller;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(get_Brag)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
