use bitcoin::Transaction;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::transaction::TransactionType;

#[derive(Debug, Clone, Copy, EnumString, Display, PartialEq, Eq)]
pub enum RegisterStatus {
    #[strum(serialize = "not_exist")]
    NotExist,
    #[strum(serialize = "stake_tx_ready_to_submit")]
    StakeTxReadyToSubmit,
    #[strum(serialize = "stake_tx_submitted")]
    StakeTxSubmitted,
    #[strum(serialize = "stake_tx_confirmed")]
    StakeTxConfirmed,
    #[strum(serialize = "registered")]
    Registered,
    #[strum(serialize = "unsigned")]
    Unsigned,
    #[strum(serialize = "unregistered")]
    Unregistered,
    #[strum(serialize = "challenging")]
    Challenging,
    #[strum(serialize = "slashed")]
    Slashed,
    #[strum(serialize = "redeemed")]
    Redeemed,
    #[strum(serialize = "removed")]
    Removed,
    #[strum(serialize = "failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub validator_key: String,
    pub register_pk: String,
    pub committee_id: u32,
    pub stake_tx: Transaction,
    pub assert_tx_hex: String,
    pub challenge_tx_hex: String,
    pub synthesiser_circuit_id: u32,
}

impl RegisterInfo {
    pub fn new(
        validator_key: &str,
        register_pk: &str,
        committee_id: u32,
        stake_tx: Transaction,
        assert_tx_hex: &str,
        challenge_tx_hex: &str,
        synthesiser_circuit_id: u32,
    ) -> Self {
        Self {
            validator_key: validator_key.to_string(),
            register_pk: register_pk.to_string(),
            committee_id,
            stake_tx,
            assert_tx_hex: assert_tx_hex.to_string(),
            challenge_tx_hex: challenge_tx_hex.to_string(),
            synthesiser_circuit_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QueryAssertTxReq {
    pub validator_key: String,
}

impl QueryAssertTxReq {
    pub fn new(validator_key: &str) -> Self {
        Self {
            validator_key: validator_key.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircuitTx {
    pub vk_hash: String,
    pub tx_type: TransactionType,
    pub tx_hex: String,
}

impl CircuitTx {
    pub fn new(vk_hash: &str, tx_type: TransactionType, tx_hex: &str) -> Self {
        Self {
            vk_hash: vk_hash.to_string(),
            tx_type,
            tx_hex: tx_hex.to_string(),
        }
    }
}
