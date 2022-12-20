use libp2p::{floodsub::Topic, identity, PeerId};
use once_cell::sync::Lazy;

/// # Path to the recipes
/// Located in hardcoded path
pub const STORAGE_FILE_PATH: &str = "./recipes.json";
/// # Keys constant
pub static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
/// # Peer Id constant
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
/// # Topic constant
pub static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));

#[cfg(test)]
mod tests {
    use crate::constants::STORAGE_FILE_PATH;

    #[test]
    fn test_storage_file_path() {
        assert_eq!(STORAGE_FILE_PATH, "./recipes.json");
    }
}
