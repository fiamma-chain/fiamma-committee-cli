use bitcoin::Txid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterNodeError {
    #[error("generate multisig address failed with error: {0}")]
    GenerateMultisigAddressError(String),
    #[error("recover register's public key failed with error: {0}")]
    RecoverPublicKeyError(String),
    #[error("register id of validator {0} is missing")]
    RegisterIDNotFound(String),
    #[error("public key of validator {0} is missing")]
    RegisterPkNotFound(String),
    #[error("register's validator key has been registered: {0}")]
    ValidatorKeyRegistered(String),
    #[error("stake tx has invalid locktime {0}")]
    StakeTxInvalidLockTime(u32),
    #[error("stake tx has invalid version {0}")]
    StakeTxInvalidVersion(i32),
    #[error("stake tx is missing, validator_key {0}")]
    StakeTxMissing(String),
    #[error("stake tx misses output")]
    StakeTxMissingOutput,
    #[error("stake tx has invalid stake value {0}")]
    StakeTxInvalidStakeValue(u64),
    #[error("stake tx has invalid connector value {0}")]
    StakeTxInvalidConnectorValue(u64),
    #[error("stake tx has invalid output script pubkey {0}")]
    StakeTxInvalidScriptPubkey(String),
    #[error("assert tx of validator {0} is missing")]
    AssertTxNotFound(String),
    #[error("assert tx has invalid version {0}")]
    AssertTxInvalidVersion(i32),
    #[error("assert tx has invalid locktime {0}")]
    AssertTxInvalidLockTime(u32),
    #[error("assert tx has invalid sequence {0}")]
    AssertTxInvalidSequence(u32),
    #[error("assert tx misses input")]
    AssertTxMissingInput,
    #[error("assert tx has invalid previous output, txid {txid}, vout {vout}")]
    AssertTxInvalidPreviousOutput { txid: Txid, vout: u32 },
    #[error("assert tx output not empty")]
    AssertTxOutputNotEmpty,
    #[error("assert tx has invalid input count {0}")]
    AssertTxInvalidInput(usize),
    #[error("assert tx is not segwit transaction")]
    AssertTxNotSegwitTx,
    #[error("assert tx has invalid witness count {0}")]
    AssertTxInvalidWitnessCount(usize),
    #[error("assert tx's witness has invalid script pubkey {0}")]
    AssertTxWitnessInvalidScriptPubKey(String),
    #[error("assert tx failed to recover register's signature with error: {0}")]
    AssertTxRecoverSignatureFailed(String),
    #[error("assert tx failed to verify register's signature with error: {0}")]
    AssertTxVerifySignatureFailed(String),
    #[error("output {0} of assert tx is missing")]
    AssertTxMissingOutput(u32),
    #[error("challenge tx has invalid version {0}")]
    ChallengeTxInvalidVersion(i32),
    #[error("challenge tx has invalid locktime {0}")]
    ChallengeTxInvalidLockTime(u32),
    #[error("challenge tx has invalid sequence {0}")]
    ChallengeTxInvalidSequence(u32),
    #[error("challenge tx misses input")]
    ChallengeTxMissingInput,
    #[error("challenge tx has invalid previous output, txid {txid}, vout {vout}")]
    ChallengeTxInvalidPreviousOutput { txid: Txid, vout: u32 },
    #[error("challenge tx output not empty")]
    ChallengeTxOutputNotEmpty,
    #[error("challenge tx has invalid input count {0}")]
    ChallengeTxInvalidInput(usize),
    #[error("challenge tx is not segwit transaction")]
    ChallengeTxNotSegwitTx,
    #[error("challenge tx has invalid witness count {0}")]
    ChallengeTxInvalidWitnessCount(usize),
    #[error("challenge tx's witness has invalid script pubkey {0}")]
    ChallengeTxWitnessInvalidScriptPubKey(String),
    #[error("challenge tx failed to recover register's signature with error: {0}")]
    ChallengeTxRecoverSignatureFailed(String),
    #[error("challenge tx failed to verify register's signature with error: {0}")]
    ChallengeTxVerifySignatureFailed(String),
    #[error("disprove tx has invalid version {0}")]
    DisproveTxInvalidVersion(i32),
    #[error("disprove tx has invalid locktime {0}")]
    DisproveTxInvalidLockTime(u32),
    #[error("disprove tx has invalid sequence {0}")]
    DisproveTxInvalidSequence(u32),
    #[error("disprove tx misses input")]
    DisproveTxMissingInput,
    #[error("disprove tx has invalid previous output, txid {txid}, vout {vout}")]
    DisproveTxInvalidPreviousOutput { txid: Txid, vout: u32 },
    #[error("disprove tx output not empty")]
    DisproveTxOutputNotEmpty,
    #[error("disprove tx has invalid input count {0}")]
    DisproveTxInvalidInput(usize),
    #[error("disprove tx is not segwit transaction")]
    DisproveTxNotSegwitTx,
    #[error("disprove tx has invalid witness count {0}")]
    DisproveTxInvalidWitnessCount(usize),
    #[error("disprove tx's witness has invalid script pubkey {0}")]
    DisproveTxWitnessInvalidScriptPubKey(String),
    #[error("disprove tx failed to recover register's signature with error: {0}")]
    DisproveTxRecoverSignatureFailed(String),
    #[error("disprove tx failed to verify register's signature with error: {0}")]
    DisproveTxVerifySignatureFailed(String),
    #[error("compute sighash p2wsh failed with error: {0}")]
    ComputeSigHashP2wshFailed(String),
    #[error("failed to get tweaked public key of committee")]
    TweakedCommitteeKeyNotFound,
    #[error("private key of committee {0} is missing")]
    CommitteePrivatekeyNotFound(u32),
    #[error("invalid disprove tx count, expect {0} but get {1}")]
    InvalidDisproveTxCount(u32, u32),
    #[error("verifier key of circuit hash {0} not found")]
    CircuitVKNotFound(String),
}

#[derive(Debug, Error)]
pub enum ChallengeError {
    #[error("register id of validator {0} is missing")]
    RegisterIDNotFound(String),
    #[error("stake tx of register {0} is missing")]
    StakeTxNotFound(u32),
    #[error("public key of register {0} is missing")]
    PublicKeyNotFound(u32),
    #[error("public key parse failed with error {0}")]
    PublicKeyParseFailed(String),
    #[error("assert tx of register {0} is missing")]
    AssertTxNotFound(String),
    #[error("assert tx of proof {0} is missing")]
    AssertTxOfProofNotFound(String),
    #[error("challenge tx of register {0} is missing")]
    ChallengeTxNotFound(String),
    #[error("challenge id of proof {0} is missing")]
    ChallengeIDNotFound(String),
    #[error("disprove tx of register {0} is missing")]
    DisproveTxNotFound(String),
    #[error("private key of committee {0} is missing")]
    CommitteePrivatekeyNotFound(u32),
    #[error("assert tx's node signature of register {0} is missing")]
    AssertTxNodeSignatureNotFound(String),
    #[error("assert tx's public script of register {0} is missing")]
    AssertTxPublicScriptNotFound(String),
    #[error("challenge tx failed to desearialize due to error {0}")]
    ChallengeTxDeserializeError(String),
    #[error("failed to query challenge data of proof {0}")]
    QueryChallengeDataFailed(String),
    #[error("Challenge with proof id: {0} already exists")]
    ChallengeExists(String),
    #[error("Challenge raw committee tx is not ready")]
    ChallengeRawCommitteeTxNotReady,
    #[error("Circuit of hash {0} is not ready or exist")]
    CircuitNotRegistered(String),
}

#[derive(Debug, Error)]
pub enum DisproveError {
    #[error("generate multisig address failed with error: {0}")]
    GenerateMultisigAddressError(String),
    #[error("failed to parse bitcoin network from config with err: {0}")]
    NetworkParseError(String),
    #[error("proof id {0} is missing")]
    ProofIDNotFound(String),
    #[error("failed to parse challenger's bitcoin address {0}")]
    ChallengerAddressParseError(String),
    #[error("assert tx of proof {0} is missing")]
    AssertTxNotFound(String),
    #[error("disprove tx of proof {0} is missing")]
    DisproveTxNotFound(String),
    #[error("challenge id of proof {0} is missing")]
    ChallengeIDNotFound(String),
    #[error("register id of proof {0} is missing")]
    RegisterIDNotFound(String),
    #[error("register public key of register {0} is missing")]
    RegisterKeyNotFound(u32),
    #[error("proof id {0} is not ready to disprove")]
    AssertTxIsNotConfirmed(String),
    #[error("script {0} is invalid")]
    ScriptInvalid(usize),
    #[error("witness {0} is invalid")]
    WitnessInvalid(usize),
    #[error("wrong script index {0}")]
    WrongScriptInedx(usize),
    #[error("failed to create control block")]
    FailedToCreateControl,
    #[error("failed to get tweaked public key of committee")]
    TweakedCommitteeKeyNotFound,
    #[error("failed to get tweaked public key of register {0}")]
    TweakedRegisterNotFound(String),
}

#[derive(Debug, Error)]
pub enum CircuitError {
    #[error("invalid circuit with hash {0}")]
    InvalidCircuitHash(String),
    #[error("circuit with hash {0} already exists")]
    CircuitAlreadyExists(String),
    #[error("circuit with hash {0} not exists")]
    CircuitNotExists(String),
    #[error("circuit with hash {0} is invalid")]
    CircuitInvalid(String),
}
