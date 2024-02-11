use candid::{CandidType, Result};
use candid::utils::ArgumentDecoder;
use ic_cdk::storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const IDENTITIES_KEY: &str = "identities";

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct Identity {
    name: String,
    age: u32,
    licenseID: String,
    verifiedCerts: String,
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct IdentityMap(HashMap<u64, Identity>);

impl IdentityMap {
    fn new() -> Self {
        IdentityMap(HashMap::new())
    }

    fn save(&self) {
        storage::stable_save((IDENTITIES_KEY, &self.0));
    }

    fn load() -> Self {
        let data: Option<HashMap<u64, Identity>> = storage::stable_restore().expect("Failed to restore data");
        data.map_or_else(|| IdentityMap::new(), IdentityMap)
    }

    fn update_identity(&mut self, id: u64, updated_identity: Identity) -> Result {
        if let Some(existing_identity) = self.0.get_mut(&id) {
            // Update the fields of existing_identity with updated_identity
            *existing_identity = updated_identity;
            self.save();
            Ok(())
        } else {
            Err(candid::Error::Subtype("Identity not found".to_string()))
        }
    }
}

impl<'de> ArgumentDecoder<'de> for Option<HashMap<u64, Identity>> {
    fn decode(value: &'de dyn candid::CandidType) -> Result<Self> {
        if let Some(data) = value.as_any().downcast_ref::<Option<HashMap<u64, Identity>>>() {
            Ok(data.clone())
        } else {
            Err("Failed to decode Option<HashMap<u64, Identity>>".to_string())
        }
    }
}


#[export_name = "canister_update register_identity"]
fn register_identity(identity: Identity) -> Result {
    let _caller = ic_cdk::caller();
    let mut identities = IdentityMap::load();

    // Process and save the identity data
    identities.save();
    Ok(())
}

#[export_name = "canister_query get_identity"]
fn get_identity(id: u64) -> Option<Identity> {
    let identities = IdentityMap::load();
    identities.0.get(&id).cloned()
}

#[export_name = "canister_update update_identity"]
fn update_identity(id: u64, updated_identity: Identity) -> Result {
    let mut identities = IdentityMap::load();
    // Handle the result of the update operation
    identities.update_identity(id, updated_identity)?;
    Ok(())
}

#[export_name = "canister_update delete_identity"]
fn delete_identity(id: u64) -> Result {
    Ok(())
}
