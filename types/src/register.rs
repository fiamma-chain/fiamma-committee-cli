use bitcoin::Transaction;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, EnumString, Display, PartialEq, Eq)]
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
    // pub disprove_tx_hex: String,
}

impl RegisterInfo {
    pub fn new(
        validator_key: &str,
        register_pk: &str,
        committee_id: u32,
        stake_tx: Transaction,
        assert_tx_hex: &str,
        challenge_tx_hex: &str,
        // disprove_tx_hex: &str,
    ) -> Self {
        Self {
            validator_key: validator_key.to_string(),
            register_pk: register_pk.to_string(),
            committee_id,
            stake_tx,
            assert_tx_hex: assert_tx_hex.to_string(),
            challenge_tx_hex: challenge_tx_hex.to_string(),
            // disprove_tx_hex: disprove_tx_hex.to_string(),
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
