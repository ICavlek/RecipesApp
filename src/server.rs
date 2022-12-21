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
use tokio::{io::AsyncBufReadExt, sync::mpsc};

pub struct Server {
    pub client: Client,
}

impl Server {
    #[tokio::main]
    pub async fn start(&self) {
        let (response_sender, mut response_rcv) = mpsc::unbounded_channel::<ListResponse>();

        let mut behaviour = RecipeBehaviour {
            floodsub: Floodsub::new(self.client.peer_id),
            mdns: Mdns::new(Default::default())
                .await
                .expect("can create mdns"),
            response_sender: response_sender,
            peer_id: self.client.peer_id,
        };

        behaviour.floodsub.subscribe(self.client.topic.clone());

        let auth_keys = Keypair::<X25519Spec>::new()
            .into_authentic(&self.client.keys)
            .expect("can create auth keys");

        let transp = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated()) // XX Handshake pattern, IX exists as well and IK - only XX currently provides interop with other libp2p impls
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        let mut swarm = SwarmBuilder::new(transp, behaviour, self.client.peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

        Swarm::listen_on(
            &mut swarm,
            "/ip4/0.0.0.0/tcp/0"
                .parse()
                .expect("can get a local socket"),
        )
        .expect("swarm can be started");

        loop {
            let evt = {
                tokio::select! {
                    line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                    response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists"))),
                    _ = swarm.select_next_some() => None,
                }
            };

            if let Some(event) = evt {
                match event {
                    EventType::Response(resp) => {
                        let json = serde_json::to_string(&resp).expect("can jsonify response");
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(self.client.topic.clone(), json.as_bytes());
                    }
                    EventType::Input(line) => match line.as_str() {
                        "ls p" => Server::handle_list_peers(&mut swarm).await,
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
