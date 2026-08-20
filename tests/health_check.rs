use std::net::TcpListener;

#[tokio::test]
async fn health_check_works(){
    let address = format!("http://{}/health_check", spawn_app());

    let client = reqwest::Client::new();

    let res = client.get(address)
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().expect("failed to receive local address").port();
    
    let server = newsletter::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{}", port)
}

