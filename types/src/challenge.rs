use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChallengeRequest {
    pub proof_id: String,
}

impl ChallengeRequest {
    pub fn new(proof_id: &str) -> Self {
        Self {
            proof_id: proof_id.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FinishChallengeRequest {
    pub proof_id: String,
    pub filled_challenge_tx: String,
}

impl FinishChallengeRequest {
    pub fn new(proof_id: &str, challenge_tx: &str) -> Self {
        Self {
            proof_id: proof_id.to_string(),
            filled_challenge_tx: challenge_tx.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChallengeInfoRes {
    pub proof_id: String,
    pub status: ChallengeStatus,
    pub challenge_txid: Option<String>,
    pub assert_txid: Option<String>,
    pub disprove_txid: Option<String>,
}

#[derive(Debug, Clone, EnumString, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChallengeStatus {
    #[strum(serialize = "challenge_not_exist")]
    ChallengeNotExist,
    #[strum(serialize = "challenge_created")]
    ChallengeCreated,
    #[strum(serialize = "partial_assert_tx_ready")]
    PartialAssertTxReady,
    #[strum(serialize = "challenge_tx_ready_to_submit")]
    ChallengeTxReadyToSubmit,
    #[strum(serialize = "challenge_tx_submitted")]
    ChallengeTxSubmitted,
    #[strum(serialize = "challenge_tx_confirmed")]
    ChallengeTxConfirmed,
    #[strum(serialize = "assert_tx_ready_to_submit")]
    AssertTxReadyToSubmit,
    #[strum(serialize = "assert_tx_submitted")]
    AssertTxSubmitted,
    #[strum(serialize = "assert_tx_confirmed")]
    AssertTxConfirmed,
    #[strum(serialize = "disprove_tx_ready_to_handle")]
    DisproveTxReadyToHandle,
    #[strum(serialize = "disprove_tx_handling")]
    DisproveTxHandling,
    #[strum(serialize = "disprove_tx_ready_to_submit")]
    DisproveTxReadyToSubmit,
    #[strum(serialize = "disprove_tx_submitted")]
    DisproveTxSubmitted,
    #[strum(serialize = "disprove_tx_confirmed")]
    DisproveTxConfirmed,
    #[strum(serialize = "disprove_tx_failed")]
    DisproveTxFailed,
    #[strum(serialize = "challenge_succeed")]
    ChallengeSucceed,
    #[strum(serialize = "challenge_failed")]
    ChallengeFailed,
}

pub struct ChallengeInfo {
    pub proof_id: String,
    pub challenge_id: u32,
    // pub validator_key: String,
    // pub assert_tx_id: String,
    // pub disprove_tx_id: String,
    // pub status: ChallengeStatus,
}
