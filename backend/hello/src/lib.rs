#[ic_cdk::query]
extern crate ic_cdk_macros;

use std::collections::HashMap;

// Include necessary encryption libraries
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use chacha20poly1305::aead::{Aead, NewAead, generic_array::GenericArray};
use chacha20poly1305::XChaCha20Poly1305;
use rand::Rng;

use ic_cdk::{export, id, caller, storage, error};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Identity {
    pub owner: ic_cdk::Principal,
    pub attributes: HashMap<String, String>,
    // Securely store encrypted credentials
    pub encrypted_credentials: Vec<u8>,
}

#[ic_cdk::canister]
mod canister {
    // Define the key for storing identities in the canister's memory.
    const IDENTITIES_KEY: &str = "identities";

    #[init]
    pub fn init() {}

    #[export]
    pub fn register(public_key: String) -> Result<u64, String> {
        // Create a new identity and return its unique identifier
        let id = id().into();
        let new_identity = Identity {
            owner: caller(),
            attributes: HashMap::new(),
            encrypted_credentials: Vec::new(),
        };
        
        // Retrieve existing identities from storage
        let mut identities: HashMap<u64, Identity> = match storage::get(IDENTITIES_KEY) {
            Some(data) => serde_cbor::from_slice(&data).map_err(|e| e.to_string())?,
            None => HashMap::new(),
        };

        // Check if the identity already exists
        if identities.contains_key(&id) {
            return Err("Identity already exists".to_string());
        }

        // Store the new identity
        identities.insert(id, new_identity);
        storage::put(IDENTITIES_KEY, &serde_cbor::to_vec(&identities).map_err(|e| e.to_string())?);

        Ok(id)
    }

    #[export]
    pub fn update_attribute(id: u64, key: String, value: String) -> Result<(), String> {
        // Retrieve the user's identity
        let identities: HashMap<u64, Identity> = match storage::get(IDENTITIES_KEY) {
            Some(data) => serde_cbor::from_slice(&data).map_err(|e| e.to_string())?,
            None => return Err("Identity not found".to_string()),
        };

        let mut identity = identities.get(&id).ok_or_else(|| "Identity not found".to_string())?.clone();

        // Update the attribute
        identity.attributes.insert(key, value);

        // Store the updated identity
        identities.insert(id, identity);
        storage::put(IDENTITIES_KEY, &serde_cbor::to_vec(&identities).map_err(|e| e.to_string())?);

        Ok(())
    }

    #[export]
    pub fn get_identity(id: u64) -> Result<Option<Identity>, String> {
        // Retrieve the identity based on its identifier
        let identities: HashMap<u64, Identity> = match storage::get(IDENTITIES_KEY) {
            Some(data) => serde_cbor::from_slice(&data).map_err(|e| e.to_string())?,
            None => return Ok(None),
        };

        let identity = identities.get(&id).cloned();

        // Remove credentials before returning
        let identity_without_credentials = identity.map(|mut i| {
            i.encrypted_credentials.clear();
            i
        });

        Ok(identity_without_credentials)
    }
}








// Always include this at the end of your file
ic_cdk::export_candid!();
