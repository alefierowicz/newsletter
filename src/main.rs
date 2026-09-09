use std::net::TcpListener;

use newsletter::telemetry::{get_subscriber, init_subscriber};
use newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // tracing config
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // app config
    let configuration = get_configuration().expect("failed to load config from file");
    let connection_pool = PgPool::connect(&configuration.db.connection_string())
        .await
        .expect("failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port); // for future referrence do NOT add http:// before the address proper
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
