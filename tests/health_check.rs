#[tokio::test]
async fn test_health_check() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute the request");

    println!("{response:?}");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() {
    let server = millennium_falcon::infrastructure_service::run("127.0.0.1:8080")
        .expect("failed to run server");
    let _ = tokio::spawn(server);
}
