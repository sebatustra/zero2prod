use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    log::info!(
        "Adding '{}' '{} as a new subscriber",
        form.email,
        form.name
    );
    log::info!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to eecute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}