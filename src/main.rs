use news_letter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0");
    let port = listener.local_addr().unwrap().port();

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run("127.0.0.1:{}", port)?.await
}