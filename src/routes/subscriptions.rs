use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[expect(dead_code)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    let rid = Uuid::new_v4();

    tracing::info!(
        "[{rid}] attempting to save user: {}, {} to the db...",
        form.name,
        form.email
    );

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
        Ok(_) => {
            tracing::info!("[{rid}] details saved successfully");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("[{rid}] failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
