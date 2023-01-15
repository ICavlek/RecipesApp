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
    let name = match std::env::args().nth(1) {
        Some(name) => name,
        None => {
            println!("Please provide a name as the first argument");
            return;
        }
    };

    let client = Client::new(name.as_str());
    let mut server = Server::new(client).await;
    server.start_listen().await;
    server.handle_events().await;
}
