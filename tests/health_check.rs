// 'tokio::test' is the testing equivalent of 'tokio::main'
// It also spares you from having to specify the #[test] attribute

// To inspect code generated: 
// cargo expand --test health_check <- name of test file 
#[tokio::test]
async fn health_check_works() {
  // spawn_app is dependent on our application code
  // Every test is agnostic 
  spawn_app().await.expect("Failed to spawn our app. ");
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
async fn spawn_app() -> Result<(), std::io::Error> {
  let server = news_letter::run().expect("Failed to bind address");

  let _ tokio::spawn(server);
}
