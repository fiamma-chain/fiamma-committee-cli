use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DisproveRequest {
    pub proof_id: String,
    pub script_index: usize,
    pub reward_addr: String,
}

impl DisproveRequest {
    pub fn new(proof_id: &str, script_index: usize, reward_addr: &str) -> Self {
        Self {
            proof_id: proof_id.to_string(),
            script_index,
            reward_addr: reward_addr.to_string(),
        }
    }
}
