use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId,
};
use tokio::sync::mpsc;

use crate::messages::ListResponse;

#[derive(NetworkBehaviour)]
pub struct RecipeBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub response_sender: mpsc::UnboundedSender<ListResponse>,
    #[behaviour(ignore)]
    pub peer_id: PeerId,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for RecipeBehaviour {
    fn inject_event(&mut self, _: FloodsubEvent) {}
}

impl NetworkBehaviourEventProcess<MdnsEvent> for RecipeBehaviour {
    fn inject_event(&mut self, _: MdnsEvent) {}
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_recipe() {
        assert_eq!(1, 1);
    }
}
