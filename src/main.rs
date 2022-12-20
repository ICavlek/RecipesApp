mod constants;
use constants::*;
mod recipe;
use libp2p::{floodsub::Floodsub, mdns::Mdns};
use log::info;
use recipe::*;
mod messages;
use messages::*;
use tokio::sync::mpsc;

/// Main function
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Peer Id: {}", PEER_ID.clone());
    let (response_sender, _) = mpsc::unbounded_channel::<ListResponse>();

    let mut behaviour = RecipeBehaviour {
        floodsub: Floodsub::new(PEER_ID.clone()),
        mdns: Mdns::new(Default::default())
            .await
            .expect("can create mdns"),
        response_sender,
    };

    behaviour.floodsub.subscribe(TOPIC.clone());
}
