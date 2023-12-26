use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::Form, Extension};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Extension(db_pool): Extension<PgPool>,
    Form(form): Form<FormData>,
) -> Response {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.email
    );
    let _request_span_guard = request_span.enter();

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
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - Successfully inserted subscription into database.",
                request_id
            );
            StatusCode::OK.into_response()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query {:?}",
                request_id,
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
