use crate::helpers::spawn_app;

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let test_app = spawn_app().await;

    // bring in reqwester
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
