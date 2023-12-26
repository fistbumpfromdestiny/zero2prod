use dotenvy::dotenv;
use env_logger::Env;
use std::env;
use zero2prod::startup::get_connection_pool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_pool = get_connection_pool(database_url);

    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Address: {}", &address);
    let listener = std::net::TcpListener::bind(address).expect("Failed to bind address.");

    run(listener, db_pool)?.await
}
