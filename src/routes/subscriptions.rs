use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// procedural macro to keep logging logic out of business logic as much as possible
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
// subscribe handles user subscription actions
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name,
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// procedural macro to keep logging logic out of business logic as much as possible
#[tracing::instrument(
    name = "saving new subscriber details to the database",
    skip(new_subscriber, pool)
)]
// isolated db logic
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    let is_too_long = s.graphemes(true).count() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbiddent_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    !(is_empty_or_whitespace || is_too_long || contains_forbiddent_characters)
}
