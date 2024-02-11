use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::{caller, id};
use ic_cdk::storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize)]
struct Identity {
    pub owner: Principal,
}

const IDENTITIES_KEY: &str = "identities";

#[update]
fn register_identity() {
    let caller = caller();
    let id = id().into();
    let new_identity = Identity {
        owner: caller,
    };

    let mut identities: HashMap<u64, Identity> = match storage::get(IDENTITIES_KEY) {
        Some(data) => serde_cbor::from_slice(&data).map_err(|e| e.to_string())?,
        None => HashMap::new(),
    };

    identities.insert(id, new_identity);
    storage::put(IDENTITIES_KEY, &serde_cbor::to_vec(&identities).map_err(|e| e.to_string())?);
}

#[update]
fn update_identity() {
    let caller = caller();
    let id = id().into();

    let mut identities: HashMap<u64, Identity> = match storage::get(IDENTITIES_KEY) {
        Some(data) => serde_cbor::from_slice(&data).map_err(|e| e.to_string())?,
        None => return,
    };

    if let Some(identity) = identities.get_mut(&id) {
        identity.owner = caller;
    }

    storage::put(IDENTITIES_KEY, &serde_cbor::to_vec(&identities).map_err(|e| e.to_string())?);
}

#[query]
fn get_identity(id: u64) -> Result<Option<Identity>, String> {
    let identities: HashMap<u64, Identity> = match storage::get(IDENTITIES_KEY) {
        Some(data) => serde_cbor::from_slice(&data).map_err(|e| e.to_string())?,
        None => return Ok(None),
    };

    Ok(identities.get(&id).cloned())
}

#[update]
fn delete_identity(id: u64) {
    let mut identities: HashMap<u64, Identity> = match storage::get(IDENTITIES_KEY) {
        Some(data) => serde_cbor::from_slice(&data).map_err(|e| e.to_string())?,
        None => return,
    };

    identities.remove(&id);

    storage::put(IDENTITIES_KEY, &serde_cbor::to_vec(&identities).map_err(|e| e.to_string())?);
}

#[cfg(test)]
mod tests {
}
