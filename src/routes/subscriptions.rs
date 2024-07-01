use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(_: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}