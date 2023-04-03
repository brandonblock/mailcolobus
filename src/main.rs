use mailcolobus::configuration::get_configuration;
use mailcolobus::startup::run;
use mailcolobus::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initiliaze log subscriber for telemetry
    let subscriber = get_subscriber("mailcolubus".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Panic if we can't get our configs
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    // Start listener
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    tracing::info!("Starting on port {}", port);
    run(listener, connection)?.await
}
