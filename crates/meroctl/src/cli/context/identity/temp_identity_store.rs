use std::collections::HashMap;
use std::sync::Mutex;

use calimero_primitives::identity::{PrivateKey, PublicKey};
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct IdentityPair {
    pub public: PublicKey,
    pub private: PrivateKey,
}

pub static TEMP_IDENTITY_STORE: Lazy<Mutex<HashMap<u8, IdentityPair>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn store_identity(slot: u8, identity: IdentityPair) -> bool {
    if slot >= 32 {
        return false;
    }
    let mut store = TEMP_IDENTITY_STORE.lock().unwrap();
    store.insert(slot, identity);
    true
}

pub fn get_identity_by_public(public_key: &PublicKey) -> Option<PrivateKey> {
    let store = TEMP_IDENTITY_STORE.lock().unwrap();
    store.values().find_map(|pair| {
        if &pair.public == public_key {
            Some(pair.private.clone())
        } else {
            None
        }
    })
}
