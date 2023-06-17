use actix_web::{web, App, HttpServer, Responder, HttpRequest};

async fn greet(req: HttpRequest) -> impl Responder {
    // match_info -> 
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    println!("Hello, world!");
}
