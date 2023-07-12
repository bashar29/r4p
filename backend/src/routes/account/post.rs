use actix_web::{post, HttpResponse, Responder};

#[post("/accounts")]
async fn create_account() -> impl Responder {
    HttpResponse::Ok().finish()
}
