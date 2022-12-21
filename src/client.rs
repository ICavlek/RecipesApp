use libp2p::{floodsub::Topic, identity, PeerId};
use log::info;

pub struct Client {
    pub keys: identity::Keypair,
    pub peer_id: PeerId,
    pub topic: Topic,
}

impl Client {
    pub fn new() -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        info!("New peer with Id: {}", peer_id);
        Self {
            keys: keypair,
            peer_id: peer_id,
            topic: Topic::new("recipes"),
        }
    }
}
