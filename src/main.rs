use actix_web::{App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::echo)
            .service(api::break_hash)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}