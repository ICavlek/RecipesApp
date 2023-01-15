mod client;
mod messages;
mod recipe;
mod server;

use client::*;
use server::*;

/// Main function
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let john = Client::new("john");
    let mut server = Server::new(john).await;
    server.start_listen().await;
    server.handle_events().await;
}
