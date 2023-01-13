use crate::{
    client::Client,
    messages::{EventType, ListResponse},
    recipe::RecipeBehaviour,
};
use libp2p::{
    core::upgrade,
    floodsub::Floodsub,
    futures::StreamExt,
    mdns::Mdns,
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use log::{error, info};
use std::collections::HashSet;
use tokio::{
    io::AsyncBufReadExt,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

pub struct Server {
    pub client: Client,
    response_receiver: UnboundedReceiver<ListResponse>,
    swarm: Swarm<RecipeBehaviour>,
}

impl Server {
    pub async fn new(client: Client) -> Self {
        let (response_sender, response_receiver): (
            UnboundedSender<ListResponse>,
            UnboundedReceiver<ListResponse>,
        ) = mpsc::unbounded_channel::<ListResponse>();

        let mut behaviour = RecipeBehaviour {
            floodsub: Floodsub::new(client.peer_id),
            mdns: Mdns::new(Default::default())
                .await
                .expect("can create mdns"),
            response_sender: response_sender.clone(),
            peer_id: client.peer_id,
        };

        behaviour.floodsub.subscribe(client.topic.clone());

        let auth_keys = Keypair::<X25519Spec>::new()
            .into_authentic(&client.keys)
            .expect("can create auth keys");

        let transp = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        let swarm: Swarm<RecipeBehaviour> = SwarmBuilder::new(transp, behaviour, client.peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        Self {
            client: client,
            response_receiver: response_receiver,
            swarm: swarm,
        }
    }

    pub async fn start_listen(&mut self) {
        Swarm::listen_on(
            &mut self.swarm,
            "/ip4/0.0.0.0/tcp/0"
                .parse()
                .expect("can get a local socket"),
        )
        .expect("swarm can be started");
    }

    pub async fn handle_events(&mut self) {
        let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
        loop {
            let evt = {
                tokio::select! {
                    line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                    response = self.response_receiver.recv() => Some(EventType::Response(response.expect("response exists"))),
                    _ = self.swarm.select_next_some() => None,
                }
            };

            if let Some(event) = evt {
                match event {
                    EventType::Response(resp) => {
                        let json = serde_json::to_string(&resp).expect("can jsonify response");
                        self.swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(self.client.topic.clone(), json.as_bytes());
                    }
                    EventType::Input(line) => match line.as_str() {
                        "ls p" => self.handle_list_peers().await,
                        "exit" => break,
                        _ => error!("unknown command"),
                    },
                }
            }
        }
    }

    async fn handle_list_peers(&mut self) {
        info!("Discovered Peers:");
        let nodes = self.swarm.behaviour().mdns.discovered_nodes();
        let mut unique_peers = HashSet::new();
        for peer in nodes {
            unique_peers.insert(peer);
        }
        unique_peers.iter().for_each(|p| info!("{}", p));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_start_listen() {
        let john = Client::new();
        let mut server = Server::new(john).await;
        server.start_listen().await;
    }

    #[tokio::test]
    async fn test_handle_list_peers() {
        let john = Client::new();
        let mut server = Server::new(john).await;
        let input = "ls p\nexit\n";
        let input = std::io::Cursor::new(input.as_bytes());
        let mut input = tokio::io::BufReader::new(input).lines();

        server.start_listen().await;
        loop {
            let evt = {
                tokio::select! {
                    line = input.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                    response = server.response_receiver.recv() => Some(EventType::Response(response.expect("response exists"))),
                    _ = server.swarm.select_next_some() => None,
                }
            };

            if let Some(event) = evt {
                match event {
                    EventType::Response(resp) => {
                        let json = serde_json::to_string(&resp).expect("can jsonify response");
                        server
                            .swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(server.client.topic.clone(), json.as_bytes());
                    }
                    EventType::Input(line) => match line.as_str() {
                        "ls p" => server.handle_list_peers().await,
                        "exit" => break,
                        _ => error!("unknown command"),
                    },
                }
            }
        }
    }
}
