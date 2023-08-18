use crate::fixture::{send, TestApp};

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;
    let client = reqwest::Client::new();
    let response = send(client.get(format!("{}/health_check", app.address))).await;

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
