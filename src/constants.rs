use libp2p::{floodsub::Topic, identity, PeerId};
use once_cell::sync::Lazy;

/// Constants Title
///
/// # Constants
///
/// Constants description
pub const STORAGE_FILE_PATH: &str = "./recipes.json";
pub static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));
