use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use controller::{health_check, get_brag,post_brag};
mod controller;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(get_brag)
            .service(post_brag)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
