use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = format!("http://{}/health_check", spawn_app());

    let client = reqwest::Client::new();

    let res = client
        .get(address)
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener
        .local_addr()
        .expect("failed to receive local address")
        .port();

    let server = newsletter::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let address: String = format!("http://{}/subscriptions", spawn_app());
    let client = reqwest::Client::new();

    let body = "name=big%20slime&email=big_slime%40gmail.com";
    let response = client
        .post(&address)
        .header("Content-Type", "application/x-www-form-unencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_bad_data() {
    let address: String = format!("http://{}/subscriptions", spawn_app());
    let client = reqwest::Client::new();

    let test_cases: Vec<(&str, &str)> = vec![
        ("missing email", "name=big%20slime"),
        ("missing name", "email=big_slime%40gmail.com&ping=pong"),
        ("missing both", ""),
    ];

    for (error_message, invalid_body) in test_cases {
        let response = client
            .post(&address)
            .header("Content-Type", "application/x-www-form-unencoded")
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
