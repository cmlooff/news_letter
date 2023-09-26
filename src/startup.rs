use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;

// No longer a binary entrypoint, there we mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
  // Wrap connection in a smart pointer
  let db_pool = Data::new(db_pool);

  let server = HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscribe))
      .app_data(db_pool.clone())
  })
    .listen(listener)?
    .run();

  Ok(server)
}