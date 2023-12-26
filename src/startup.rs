use crate::create_routes;
use axum::routing::IntoMakeService;
use axum::Router;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use hyper::server::conn::AddrIncoming;
use hyper::Server;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    db_pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app = create_routes(db_pool);
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
