use crate::routes::{health_check, subscriptions};
use axum::routing::{get, post, IntoMakeService};
use axum::{Extension, Router};
use hyper::server::conn::AddrIncoming;
use hyper::Server;
use sqlx::PgPool;
use std::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing_core::Level;

pub fn run(
    listener: TcpListener,
    pool: PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
        .layer(Extension(pool))
        .layer(
            TraceLayer::new_for_http().on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Millis),
            ),
        );
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
