use std::net::TcpListener;

use newsletter::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to port");
    run(listener)?.await
}
