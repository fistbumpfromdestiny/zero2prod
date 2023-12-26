pub mod configuration;
pub mod routes;
pub mod startup;

use crate::routes::{health_check, subscriptions};
use tower_http::trace::{DefaultOnResponse, TraceLayer};

use axum::routing::post;
use axum::{routing::get, Extension, Router};
use metadata::Level;
use sqlx::PgPool;
use tower_http::LatencyUnit;
use tracing_core::metadata;

fn create_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
        .layer(Extension(db_pool))
        .layer(
            TraceLayer::new_for_http().on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Millis),
            ),
        )
}
