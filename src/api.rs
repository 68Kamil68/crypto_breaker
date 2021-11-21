use actix_web::{web, post, HttpResponse, Responder};

mod serializers;

#[post("/break-hash")]
async fn break_hash(info: web::Json<serializers::UserInfo>) -> impl Responder {
    HttpResponse::Ok().body(&info.email)
}


#[post("/echo")]
async fn echo(request_data: String) -> impl Responder {
    HttpResponse::Ok().body(request_data)
}

