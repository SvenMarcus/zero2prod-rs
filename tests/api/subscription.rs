use crate::fixture::{send, TestApp};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = TestApp::spawn().await;

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = send(
        reqwest::Client::new()
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body),
    )
    .await;

    assert_eq!(200, response.status().as_u16());

    let _ = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = TestApp::spawn().await;

    let test_cases = vec![
        ("name=le%20guin", "Missing email"),
        ("email=ursula_le_guin%40gmail.com", "Missing name"),
        ("", "Missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = send(
            reqwest::Client::new()
                .post(format!("{}/subscriptions", app.address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body),
        )
        .await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
