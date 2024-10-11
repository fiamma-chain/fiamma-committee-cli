#[derive(Debug)]
pub struct PresignedTransactions {
    pub validator_key: String,
    pub circuit_hash: String,
    pub taproot_address: String,
    pub committee_assert_tx: String,
    pub committee_challenge_tx: String,
}

impl PresignedTransactions {
    pub fn new(
        validator_key: &str,
        circuit_hash: &str,
        taproot_address: &str,
        committee_assert_tx: &str,
        committee_challenge_tx: &str,
    ) -> Self {
        Self {
            validator_key: validator_key.to_string(),
            circuit_hash: circuit_hash.to_string(),
            taproot_address: taproot_address.to_string(),
            committee_assert_tx: committee_assert_tx.to_string(),
            committee_challenge_tx: committee_challenge_tx.to_string(),
        }
    }
}
