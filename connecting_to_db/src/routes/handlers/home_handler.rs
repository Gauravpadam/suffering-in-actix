use actix_web::{web, Responder, HttpResponse};

#[actix_web::get("/greet/{id}")]
pub async fn greet(user_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", user_id))
}

