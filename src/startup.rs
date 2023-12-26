use crate::routes::{health_check, subscriptions};
use axum::routing::{get, post, IntoMakeService};
use axum::{Extension, Router};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use hyper::server::conn::AddrIncoming;
use hyper::Server;
use std::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing_core::Level;

pub fn run(
    listener: TcpListener,
    db_pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
        .layer(Extension(db_pool))
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

pub fn get_connection_pool(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool.")
}
