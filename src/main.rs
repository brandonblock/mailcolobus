use mailcolobus::configuration::get_configuration;
use mailcolobus::startup::run;

use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't get our configs
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failec to connect to Postgres");

    // Start listener
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    println!("Starting on port {}", port);
    run(listener, connection)?.await
}
