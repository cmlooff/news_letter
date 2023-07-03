use news_letter::run;
use std::net::TcpListener;

// To inspect code generated: cargo expand --test health_check <- name of test file 
#[tokio::test]
async fn health_check_works() {
  // Every test is agnostic 
  let address = spawn_app();
  // Need to bring in `reqwest` to perform HTTP requests against our app
  let client = reqwest::Client::new();

  // Act
  let response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

  // Retrieve the port assigned to us by the OS
  let port = listener.local_addr().unwrap().port();
  let server = run(listener).expect("Failed to bind address");

  let _ = tokio::spawn(server);

  // Return the application address to the caller
  format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16()); // Asserting that response status is equal to 200
}