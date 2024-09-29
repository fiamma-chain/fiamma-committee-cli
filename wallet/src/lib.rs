use anyhow::Context;
use bitcoin::{ScriptBuf, Txid};
use errors::ClientError;
use signer::Signer;
use types::challenge::{ChallengeInfoRes, ChallengeRequest, FinishChallengeRequest};
use types::disprove::DisproveRequest;
use types::register::QueryAssertTxReq;
use types::{FinishRegisterRequest, RegisterRequest};
use web3_decl::jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use web3_decl::namespaces::committee::CommitteeNamespaceClient;

pub mod errors;
pub mod provider;
pub mod signer;

pub struct Wallet<P> {
    pub provider: P,
    pub signer: Signer,
}

impl Wallet<HttpClient> {
    pub fn with_http_client(
        rpc_address: &str,
        signer: Signer,
    ) -> Result<Wallet<HttpClient>, ClientError> {
        let client = HttpClientBuilder::default().build(rpc_address)?;

        Ok(Wallet {
            provider: client,
            signer,
        })
    }
}

impl<P> Wallet<P>
where
    P: CommitteeNamespaceClient + Sync,
{
    pub fn new(provider: P, signer: Signer) -> Self {
        Self { provider, signer }
    }

    pub async fn get_multi_sig_script(&self, register_pk: &str) -> anyhow::Result<ScriptBuf> {
        self.provider
            .get_multi_sig_address(register_pk)
            .await
            .context("Failed to get multiple signature account")
    }

    pub async fn get_multi_sig_script_of_proof(&self, proof_id: &str) -> anyhow::Result<ScriptBuf> {
        self.provider
            .get_multi_sig_address_of_proof(proof_id)
            .await
            .context("Failed to get multiple signature account")
    }

    pub async fn start_register(&self, request: RegisterRequest) -> anyhow::Result<u32> {
        self.provider
            .start_register(request)
            .await
            .context("Failed to register")
    }

    pub async fn finish_register(&self, request: FinishRegisterRequest) -> anyhow::Result<u32> {
        self.provider
            .finish_register(request)
            .await
            .context("Failed to register")
    }

    pub async fn start_challenge(&self, request: ChallengeRequest) -> anyhow::Result<u32> {
        self.provider
            .start_challenge(request)
            .await
            .context("Failed to start a challenge")
    }

    pub async fn challenge_status(&self, request: ChallengeRequest) -> anyhow::Result<String> {
        self.provider
            .challenge_status(request)
            .await
            .context("Failed to query challenge status")
    }

    pub async fn challenge_info(
        &self,
        request: ChallengeRequest,
    ) -> anyhow::Result<ChallengeInfoRes> {
        self.provider
            .challenge_info(request)
            .await
            .context("Failed to query challenge info")
    }

    pub async fn get_committee_challenge_tx(
        &self,
        request: ChallengeRequest,
    ) -> anyhow::Result<String> {
        self.provider
            .get_committee_challenge_tx(request)
            .await
            .context("Failed to query challenge transaction")
    }

    pub async fn get_committee_assert_tx(
        &self,
        request: QueryAssertTxReq,
    ) -> anyhow::Result<String> {
        self.provider
            .get_committee_assert_tx(request)
            .await
            .context("Failed to query assert transaction")
    }

    pub async fn finish_challenge(&self, request: FinishChallengeRequest) -> anyhow::Result<Txid> {
        self.provider
            .finish_challenge(request)
            .await
            .context("Failed to finish challenge")
    }

    pub async fn disprove(&self, request: DisproveRequest) -> anyhow::Result<()> {
        self.provider
            .disprove(request)
            .await
            .map_err(|e| panic!("failed to disprove: {}", e.to_string()))
    }
}
