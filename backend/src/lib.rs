use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

//#[actix_web::main]
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(hello).service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}
