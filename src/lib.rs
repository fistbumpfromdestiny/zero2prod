pub mod configuration;
pub mod routes;
pub mod startup;

use crate::routes::{health_check, subscriptions};

use axum::routing::post;
use axum::{routing::get, Router};

fn create_routes() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
}
