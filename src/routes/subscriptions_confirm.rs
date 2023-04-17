use actix_web::{get, web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}
#[tracing::instrument(name = "confirm a pending subscriber", skip(_parameters))]
#[get("/subscriptions/confirm")]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}