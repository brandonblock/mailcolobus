use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
