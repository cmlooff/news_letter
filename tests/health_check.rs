use news_letter::{startup::run, configuration::{get_configuration, DatabaseSettings}};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use actix_web::HttpResponse;
use uuid::Uuid;

pub async fn health_check() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

// To inspect code generated: cargo expand --test health_check <- name of test file 
#[tokio::test]
async fn health_check_works() {
  let app = spawn_app().await;
  // Need to bring in `reqwest` to perform HTTP requests against our app
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

// Launch our application in the background
async fn spawn_app() -> TestApp {
  // Set port to :0 -> This sets a random port 
  let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

  // Retrieve the port assigned to us by the OS
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let mut configuration = get_configuration().expect("Failed to read configuration");
  configuration.database.database_name = Uuid::new_v4().to_string();

  let connection_pool = configure_database(&configuration.database).await; 

  let server = run(listener, connection_pool.clone())
    .expect("Failed to bind address");
  let _ = tokio::spawn(server);

  TestApp {
    address,
    db_pool: connection_pool,
  }
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16()); // Asserting that response status is equal to 200

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
      .fetch_one(&app.db_pool)
      .await
      .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both the email and the name")
    ];

    for(invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customized error message on test failure
            "The API did note fail with a 400 Bad Request when the payload for {}.", error_message
        );
    }
}