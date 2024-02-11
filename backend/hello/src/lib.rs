use candid::{CandidType, Deserialize, Principal, Result};
use ic_cdk::{export::Principal, storage, IDLArgs};

use serde::{Deserialize as SDeserialize, Serialize};
use std::collections::HashMap;

const IDENTITIES_KEY: &str = "identities";

#[derive(CandidType, SDeserialize, Serialize)]
struct Identity {
}

#[derive(CandidType, SDeserialize, Serialize)]
struct IdentityMap(HashMap<u64, Identity>);

impl IdentityMap {
    fn new() -> Self {
        IdentityMap(HashMap::new())
    }

    fn save(&self) {
        storage::write(IDENTITIES_KEY, self);
    }

    fn load() -> Self {
        storage::read().unwrap_or_else(|_| IdentityMap::new())
    }
}

#[export_name = "canister_update register_identity"]
fn register_identity() -> Result<(), String> {
    let caller = Principal::from(*ic_cdk::caller());
    let mut identities = IdentityMap::load();


    identities.save();
    Ok(())
}

#[export_name = "canister_query get_identity"]
fn get_identity(id: u64) -> Option<Identity> {
    let identities = IdentityMap::load();


    identities.0.get(&id).cloned()
}

#[export_name = "canister_update update_identity"]
fn update_identity() -> Result<(), String> {
    let caller = Principal::from(*ic_cdk::caller());
    let mut identities = IdentityMap::load();


    identities.save();
    Ok(())
}

#[export_name = "canister_update delete_identity"]
fn delete_identity(id: u64) -> Result<(), String> {
    let mut identities = IdentityMap::load();


    identities.save();
    Ok(())
}
