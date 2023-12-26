use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::Form, Extension};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Extension(db_pool): Extension<Pool<ConnectionManager<PgConnection>>>,
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
    let connection = db_pool.get().expect("Failed to connect to the database.");
    StatusCode::OK.into_response()
}
