use std::net::TcpListener;
use news_letter::startup::run;
use news_letter::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration= get_configuration().expect("Failed to read configuration");
    // We have removed the hard-coded '8000' it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}