use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().finish()
}

// Need to mark 'run' as public.
// No longer a binary entrypoint, there we mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new( || {
    App::new()
      .route("/health_check", web::get().to(health_check))
  })
    .bind(listener)?
    .run();

  Ok(server)
}