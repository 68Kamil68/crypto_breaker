use actix_web::{post, HttpResponse, Responder};


#[post("/echo")]
async fn echo(request_data: String) -> impl Responder {
    HttpResponse::Ok().body(request_data)
}
