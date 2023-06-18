use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};

// Request Handler: Greet
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Application listens to a TCP socket 127.0.0.1:8000
    // HttpServer handles all transport level concerns
    // but now we have to handle client request -> App
    HttpServer::new(|| {
        // This is where the App logic resides (routing, middleware, etc)!
        App::new()
            .route("/", web::get().to(greet))
            .route("/user/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
