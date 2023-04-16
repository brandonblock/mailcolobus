use mailcolobus::configuration::{get_configuration, DatabaseSettings};
use mailcolobus::email_client::EmailClient;
use mailcolobus::startup::run;
use mailcolobus::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::mem::drop;
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration");

    // Create testing db name
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection = configure_database(&configuration.database).await;

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

    let server = run(listener, connection.clone(), email_client).expect("Failed to bind address");

    // dropping the await so the tests will exit
    drop(actix_web::rt::spawn(server));

    TestApp {
        address,
        db_pool: connection,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    //Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to create database");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("failed to create database");

    //Migrate database
    let connection_pool = PgPool::connect_with(config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
