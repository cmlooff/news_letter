// [tokio::test] spares you from having to specify the #[test] attribute

use news_letter::run;

// To inspect code generated: cargo expand --test health_check <- name of test file 
#[tokio::test]
async fn health_check_works() {
  // spawn_app is dependent on our application code
  // Every test is agnostic 
  spawn_app();
  // Need to bring in `reqwest` to perform HTTP requests against our app
  let client = reqwest::Client::new();

  // Act
  let response = client
    .get("http://127.0.0.1:8000/health_check")
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() {
  // Using the run method in news_letter lib
  let server = news_letter::run("127.0.0.1:0").expect("Failed to bind address");

  let _ = tokio::spawn(server);
}
