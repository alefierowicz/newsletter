use std::net::TcpListener;

use newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to load config from file");
    let connection_pool = PgPool::connect(&configuration.db.connection_string())
        .await
        .expect("failed to connect to postgres");

    let address = format!("http://127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
