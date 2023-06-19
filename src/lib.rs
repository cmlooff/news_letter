use actix_web::{web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().finish()
}

// Need to mark 'run' as public.
// No longer a binary entrypoint, there we mark it as async
// without having to use any proc-macro incantation.
pub async fn run() -> Result<(), std::io::Error> {
  HttpServer::new( || {
    App::new()
      .route("/health_check", web::get().to(health_check))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}