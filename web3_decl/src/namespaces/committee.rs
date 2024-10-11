#![allow(unused_imports)]
use bitcoin::{ScriptBuf, Txid};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use types::{
    challenge::{ChallengeInfoRes, ChallengeRequest, FinishChallengeRequest},
    circuit::RegisterCircuitRequest,
    disprove::DisproveRequest,
    register::{CircuitTx, QueryAssertTxReq},
    FinishRegisterRequest, RegisterRequest,
};

#[cfg_attr(
    all(feature = "client", feature = "server"),
    rpc(server, client, namespace = "fc")
)]
#[cfg_attr(
    all(feature = "client", not(feature = "server")),
    rpc(client, namespace = "fc")
)]
#[cfg_attr(
    all(not(feature = "client"), feature = "server"),
    rpc(server, namespace = "fc")
)]
pub trait CommitteeNamespace {
    #[method(name = "getMultiSigAddress")]
    async fn get_multi_sig_address(&self, register_pk: &str) -> RpcResult<ScriptBuf>;

    #[method(name = "getMultiSigAddressOfProof")]
    async fn get_multi_sig_address_of_proof(&self, proof_id: &str) -> RpcResult<ScriptBuf>;

    #[method(name = "startRegister")]
    async fn start_register(&self, request: RegisterRequest) -> RpcResult<u32>;

    #[method(name = "finishRegister")]
    async fn finish_register(&self, request: FinishRegisterRequest) -> RpcResult<u32>;

    #[method(name = "startChallenge")]
    async fn start_challenge(&self, request: ChallengeRequest) -> RpcResult<u32>;

    #[method(name = "challengeStatus")]
    async fn challenge_status(&self, request: ChallengeRequest) -> RpcResult<String>;

    #[method(name = "challengeInfo")]
    async fn challenge_info(&self, request: ChallengeRequest) -> RpcResult<ChallengeInfoRes>;

    #[method(name = "getCommitteeChallengeTx")]
    async fn get_committee_challenge_tx(&self, request: ChallengeRequest) -> RpcResult<String>;

    #[method(name = "getCommitteeAssertTxs")]
    async fn get_committee_assert_txs(
        &self,
        request: QueryAssertTxReq,
    ) -> RpcResult<Vec<CircuitTx>>;

    #[method(name = "finishChallenge")]
    async fn finish_challenge(&self, request: FinishChallengeRequest) -> RpcResult<Txid>;

    #[method(name = "disprove")]
    async fn disprove(&self, request: DisproveRequest) -> RpcResult<()>;

    #[method(name = "registerCircuit")]
    async fn register_circuit(&self, request: RegisterCircuitRequest) -> RpcResult<u32>;
}
