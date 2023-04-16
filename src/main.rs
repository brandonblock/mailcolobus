use mailcolobus::configuration::get_configuration;
use mailcolobus::email_client::EmailClient;
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
    let sender_email = configuration
        .email_client
        .sender()
        .expect("invalid sender email address");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );
    // Start listener
    let listener = TcpListener::bind(address)?;

    let port = listener.local_addr().unwrap().port();
    tracing::info!("Starting on port {}", port);
    run(listener, connection, email_client)?.await?;
    Ok(())
}
