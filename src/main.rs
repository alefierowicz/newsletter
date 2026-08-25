use std::net::TcpListener;

use env_logger::Env;
use newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("failed to load config from file");
    let connection_pool = PgPool::connect(&configuration.db.connection_string())
        .await
        .expect("failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port); // for future referrence do NOT add http:// before the address proper
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
