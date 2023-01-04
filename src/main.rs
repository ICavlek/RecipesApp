mod client;
mod messages;
mod recipe;
mod server;

use client::*;
use server::*;

/// Main function
fn main() {
    pretty_env_logger::init();
    let john = Client::new();
    let mut server = Server::new(john);
    server.start();
}
