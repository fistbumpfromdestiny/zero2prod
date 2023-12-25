use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub(crate) async fn health_check() -> Response {
    StatusCode::OK.into_response()
}
