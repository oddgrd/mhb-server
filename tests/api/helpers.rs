use mhb_server::startup::Application;
use sqlx::{postgres::PgConnectOptions, Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

/// Spawn a new application as a background task with a new database
/// for each test, ensuring test isolation
pub async fn spawn_app() -> TestApp {
    let db_config = DatabaseSettings::new();

    // Create and migrate the database
    let pool = configure_database(&db_config).await;

    // Build the app
    let application = Application::build(pool).expect("Failed to build application");

    // Bind with port `0` to assign a random port for each test
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind port");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    // Launch the app as a background task
    tokio::spawn(application.run_until_stopped(addr));

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{port}"),
        port,
        api_client: client,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[derive(Clone, Debug)]
struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Create a new instance of `DatabaseSettings` with sensible defaults
    /// for testing
    pub fn new() -> Self {
        DatabaseSettings {
            username: "postgres".into(),
            password: "password".into(),
            port: 5432,
            host: "localhost".into(),
            // Create a new database name for each test
            database_name: Uuid::new_v4().to_string(),
        }
    }

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}
