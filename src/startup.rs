use crate::create_routes;
use axum::routing::IntoMakeService;
use axum::Router;
use hyper::server::conn::AddrIncoming;
use hyper::Server;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app = create_routes();
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
