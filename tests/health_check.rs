use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl};
use rand::{distributions::Alphanumeric, Rng};
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::schema::subscriptions;
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

async fn spawn_app() -> TestApp {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = String::from("newsletter");
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(config.connection_string());
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let random_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let random_email = format!("{}@example.com", random_name);

    let body = format!("name={}&email={}", random_name, random_email);

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = subscriptions::table
        .select((subscriptions::email, subscriptions::name))
        .order(subscriptions::id.desc())
        .first::<(String, String)>(&mut app.db_pool.get().unwrap())
        .optional()
        .expect("Failed to fetch saved subscription.");

    if let Some(saved) = saved {
        let (email, name) = saved;
        assert_eq!(email, random_email);
        assert_eq!(name, random_name);
    } else {
        panic!("No subscription was found in the database.");
    }

    diesel::delete(subscriptions::table.filter(subscriptions::email.eq(random_email)))
        .execute(&mut app.db_pool.get().unwrap())
        .expect("Failed to delete subscription.");
}

#[tokio::test]
async fn subscribe_returs_a_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 422 Unprocessable Entity when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
