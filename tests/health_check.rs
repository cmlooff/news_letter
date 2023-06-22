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

// Launch our application in the background ~somehow~
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
