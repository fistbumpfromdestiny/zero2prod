use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Form;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub(crate) async fn subscribe(Form(_form): Form<FormData>) -> Response {
    StatusCode::OK.into_response()
}
