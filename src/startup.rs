use crate::routes::{health_check, subscriptions};
use axum::http::Request;
use axum::routing::{get, post, IntoMakeService};
use axum::{Extension, Router};
use hyper::server::conn::AddrIncoming;
use hyper::Server;
use sqlx::PgPool;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestId, RequestId};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::{LatencyUnit, ServiceBuilderExt};
use tracing_core::Level;
use uuid::Uuid;

#[derive(Clone)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();

        Some(RequestId::new(request_id.parse().unwrap()))
    }
}
pub fn run(
    listener: TcpListener,
    pool: PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
        .layer(Extension(pool))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(true)
                                .level(Level::INFO),
                        )
                        .on_response(
                            DefaultOnResponse::new()
                                .latency_unit(LatencyUnit::Micros)
                                .include_headers(true),
                        ),
                ),
        );
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
