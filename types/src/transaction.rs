use bitcoin::{consensus::encode, Transaction};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, EnumString, Display, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    #[strum(serialize = "stake_tx")]
    StakeTx,
    #[strum(serialize = "challenge_tx")]
    ChallengeTx,
    #[strum(serialize = "assert_tx")]
    AssertTx,
    #[strum(serialize = "disprove_tx")]
    DisproveTx,
}

impl From<i32> for TransactionType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::StakeTx,
            1 => Self::ChallengeTx,
            2 => Self::AssertTx,
            3 => Self::DisproveTx,
            _ => panic!("Invalid value for TransactionType"),
        }
    }
}

#[derive(Debug, EnumString, Display)]
pub enum TransactionStatus {
    #[strum(serialize = "to_be_checked")]
    ToBeChecked,
    #[strum(serialize = "to_be_submitted")]
    ToBeSubmitted,
    #[strum(serialize = "submitted")]
    Submitted,
    #[strum(serialize = "confirmed")]
    Confirmed,
    #[strum(serialize = "invalid")]
    Invalid,
    #[strum(serialize = "failed")]
    Failed,
}

#[derive(Debug, Clone)]
pub struct FiammaTransaction {
    pub tx_id: String,
    pub tx_type: TransactionType,
    pub data: String,
    pub ext_info: Option<String>,
    pub register_id: u32,
}

impl FiammaTransaction {
    pub fn new_stake_tx(raw_tx: &Transaction, register_id: u32) -> Self {
        let tx_id = raw_tx.compute_txid().to_string();
        Self {
            tx_id,
            tx_type: TransactionType::StakeTx,
            data: encode::serialize_hex(raw_tx),
            ext_info: None,
            register_id,
        }
    }

    pub fn new_challenge_tx(raw_tx: &Transaction, register_id: u32) -> Self {
        let tx_id = raw_tx.compute_txid().to_string();
        Self {
            tx_id,
            tx_type: TransactionType::ChallengeTx,
            data: encode::serialize_hex(raw_tx),
            ext_info: None,
            register_id,
        }
    }

    pub fn new_assert_tx(raw_tx: &Transaction, register_id: u32) -> Self {
        let tx_id = raw_tx.compute_txid().to_string();
        Self {
            tx_id,
            tx_type: TransactionType::AssertTx,
            data: encode::serialize_hex(raw_tx),
            ext_info: None,
            register_id,
        }
    }

    pub fn new_disprove_tx(raw_tx: &Transaction, register_id: u32) -> Self {
        let tx_id = raw_tx.compute_txid().to_string();
        Self {
            tx_id,
            tx_type: TransactionType::DisproveTx,
            data: encode::serialize_hex(raw_tx),
            ext_info: None,
            register_id,
        }
    }
}
