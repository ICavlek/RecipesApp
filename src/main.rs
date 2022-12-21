mod messages;
mod server;
use std::thread;

use server::*;
mod recipe;
use libp2p::{floodsub::Topic, identity, PeerId};
use log::info;

/// Main function
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let keys: identity::Keypair = identity::Keypair::generate_ed25519();
    let peer_id: PeerId = PeerId::from(keys.public());
    let topic: Topic = Topic::new("recipes");
    info!("Peer Id: {}", peer_id);
    let server = Server {
        keys: keys,
        topic: topic,
        peer_id: peer_id,
    };

    let thread = thread::spawn(move || {
        server.start();
    });
    thread.join().unwrap();
}
