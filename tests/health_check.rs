use newsletter::configuration::{DatabaseSettings, get_configuration};
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
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

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port.");
    let port = listener.local_addr().expect("failed to fetch port").port();

    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("failed to load config from file");
    configuration.db.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_db(&configuration.db).await;

    let server = run(listener, connection_pool.clone()).expect("failed to bind address");

    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db().expose_secret())
        .await
        .expect("failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, &config.database_name).as_str())
        .await
        .expect("failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("failed to connect to Postgres (using PgPool)");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=big%20slime&email=big_slime%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("failed to fetch saved sub");

    assert_eq!(saved.email, "big_slime@gmail.com");
    assert_eq!(saved.name, "big slime");
}

#[tokio::test]
async fn subscribe_returns_400_for_bad_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases: Vec<(&str, &str)> = vec![
        ("missing email", "name=big%20slime"),
        ("missing name", "email=big_slime%40gmail.com&ping=pong"),
        ("missing both", ""),
    ];

    for (error_message, invalid_body) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didn't respond appropriately when the payload was {}",
            error_message
        );
    }
}
