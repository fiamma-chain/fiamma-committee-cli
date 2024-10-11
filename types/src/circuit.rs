use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use strum::{Display, EnumString};

#[derive(Debug, Clone, EnumString, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitType {
    #[strum(serialize = "groth16")]
    Groth16,
    #[strum(serialize = "fflonk")]
    Fflonk,
}

#[derive(Debug, Clone, EnumString, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitStatus {
    #[strum(serialize = "not_exist")]
    NotExist,
    #[strum(serialize = "initial")]
    Initial,
    #[strum(serialize = "handling")]
    Handling,
    #[strum(serialize = "splitted")]
    Splitted,
    #[strum(serialize = "registered")]
    Registered,
    #[strum(serialize = "removed")]
    Removed,
    #[strum(serialize = "failed")]
    Failed,
}

#[derive(Debug, Clone)]
pub struct CircuitInfo {
    pub vk_hash: String,
    pub vk: Vec<u8>,
    pub circuit_type: CircuitType,
}

impl CircuitInfo {
    pub fn new(vk: &[u8], circuit_type: CircuitType) -> Self {
        Self {
            vk_hash: CircuitInfo::hash(vk, circuit_type.clone()),
            vk: vk.to_vec(),
            circuit_type,
        }
    }

    pub fn hash(vk: &[u8], circuit_type: CircuitType) -> String {
        let mut hasher = Sha256::new();
        hasher.update(vk);
        hasher.update(circuit_type.to_string());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

#[derive(Debug, Clone)]
pub struct CircuitStorage {
    pub id: u32,
    pub vk_hash: String,
    pub oid: Option<i32>,
    pub circuit_type: CircuitType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterCircuitRequest {
    pub vk: Vec<u8>,
    pub circuit_type: CircuitType,
}
