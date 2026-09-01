use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    let rid = Uuid::new_v4();

    let span = tracing::info_span!(
        "adding a new subscriber",
        %rid,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _req_span_guard = span.enter();
    let q_span = tracing::info_span!("saving new subscriber to the db...");

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
    .instrument(q_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("[{rid}] failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
