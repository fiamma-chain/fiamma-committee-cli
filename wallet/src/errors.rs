use thiserror::Error;
use web3_decl::jsonrpsee::core::ClientError as RpcError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SignerError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    #[error("Signing failed: {0}")]
    SigningFailed(String),
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Missing required field for a transaction: {0}")]
    MissingRequiredField(String),
    #[error("Signing error: {0}")]
    SigningError(#[from] SignerError),
    #[error("RPC error: {0:?}")]
    RpcError(#[from] RpcError),
    #[error("Invalid ABI File")]
    AbiParseError,
}
