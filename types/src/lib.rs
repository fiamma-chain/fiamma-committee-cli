use register::CircuitTx;
use serde::{Deserialize, Serialize};

pub mod challenge;
pub mod circuit;
pub mod committee;
pub mod constants;
pub mod disprove;
pub mod error;
pub mod file;
pub mod presigned_transactions;
pub mod register;
pub mod storage_transaction;
pub mod transaction;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub validator_key: String,
    pub public_key: String,
    pub stake_tx: Vec<u8>,
    pub assert_tx: Vec<u8>,
    pub challenge_tx: Vec<u8>,
}

impl RegisterRequest {
    pub fn new(
        validator_key: &str,
        public_key: &str,
        stake_tx: &str,
        assert_tx: &str,
        challenge_tx: &str,
    ) -> Self {
        let stake_tx = stake_tx.as_bytes().to_vec();
        let assert_tx = assert_tx.as_bytes().to_vec();
        let challenge_tx = challenge_tx.as_bytes().to_vec();
        Self {
            validator_key: validator_key.to_string(),
            public_key: public_key.to_string(),
            stake_tx,
            assert_tx,
            challenge_tx,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FinishRegisterRequest {
    pub validator_key: String,
    pub disprove_txs: Vec<CircuitTx>,
}

impl FinishRegisterRequest {
    pub fn new(validator_key: &str, disprove_txs: &[CircuitTx]) -> Self {
        Self {
            validator_key: validator_key.to_string(),
            disprove_txs: disprove_txs.to_vec(),
        }
    }
}
