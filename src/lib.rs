use actix_web::dev::Server;
use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}
