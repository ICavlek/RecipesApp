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
    #[tokio::main]
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

    #[tokio::main]
    pub async fn start(&mut self) {
        self.start_listen();
        self.handle_events().await;
    }

    fn start_listen(&mut self) {
        Swarm::listen_on(
            &mut self.swarm,
            "/ip4/0.0.0.0/tcp/0"
                .parse()
                .expect("can get a local socket"),
        )
        .expect("swarm can be started");
    }

    async fn handle_events(&mut self) {
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
                        "ls p" => Server::handle_list_peers(&mut self.swarm).await,
                        "exit" => break,
                        _ => error!("unknown command"),
                    },
                }
            }
        }
    }

    async fn handle_list_peers(swarm: &mut Swarm<RecipeBehaviour>) {
        info!("Discovered Peers:");
        let nodes = swarm.behaviour().mdns.discovered_nodes();
        let mut unique_peers = HashSet::new();
        for peer in nodes {
            unique_peers.insert(peer);
        }
        unique_peers.iter().for_each(|p| info!("{}", p));
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_server() {
        assert_eq!(1, 1);
    }
}
