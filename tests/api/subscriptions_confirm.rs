use crate::helpers::spawn_app;

#[actix_web::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    //arrange
    let test_app = spawn_app().await;

    //act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", test_app.address))
        .await
        .unwrap();

    //assert
    assert_eq!(response.status().as_u16(), 400);
}
