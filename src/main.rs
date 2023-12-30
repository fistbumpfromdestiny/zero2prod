use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

mod telemetry;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Address: {}", &address);
    let listener = std::net::TcpListener::bind(address).expect("Failed to bind address.");

    run(listener, db_pool)?.await
}
