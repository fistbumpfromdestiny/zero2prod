pub mod configuration;
pub mod models;
pub mod routes;
pub mod schema;
pub mod startup;

use crate::routes::{health_check, subscriptions};
use tower_http::trace::{DefaultOnResponse, TraceLayer};

use axum::routing::post;
use axum::{routing::get, Extension, Router};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use metadata::Level;
use tower_http::LatencyUnit;
use tracing_core::metadata;

fn create_routes(db_pool: Pool<ConnectionManager<PgConnection>>) -> Router {
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
