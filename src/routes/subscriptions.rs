use actix_web::{web, HttpResponse};
use sqlx::{types::{chrono, uuid::Uuid}, PgPool};
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>
) -> HttpResponse {
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
        .execute(db_pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            println!("Failed to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
