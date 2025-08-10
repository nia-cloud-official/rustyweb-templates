use rustyweb_core::server::start_server;

#[tokio::main]
async fn main() {
    println!("Starting RustyWeb app...");
    start_server().await;
}

