pub mod configuration;
pub mod routes;
pub mod startup;

use crate::routes::{health_check, subscriptions};

use axum::routing::post;
use axum::{routing::get, Extension, Router};
use sqlx::PgPool;

fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
        .layer(Extension(pool))
}
