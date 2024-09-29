use std::net::SocketAddr;

use serde::Deserialize;

use crate::{load_config, BYTES_IN_MB, FIAMMA_COMMITTEE_PREFIX};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ApiConfig {
    pub web3_json_rpc: Web3JsonRpcConfig,
    pub healthcheck: HealthCheckConfig,
    pub bitcoin_rpc: BitcoinRpcConfig,
    pub fiamma_rpc: FiammaRpcConfig,
}

impl ApiConfig {
    pub fn load_config() -> Result<ApiConfig, config::ConfigError> {
        Ok(ApiConfig {
            web3_json_rpc: Web3JsonRpcConfig::load_config()?,
            healthcheck: HealthCheckConfig::load_config()?,
            bitcoin_rpc: BitcoinRpcConfig::load_config()?,
            fiamma_rpc: FiammaRpcConfig::load_config()?,
        })
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Web3JsonRpcConfig {
    pub http_port: u16,
    pub http_url: String,
    pub max_batch_request_size: Option<usize>,
    pub max_response_body_size_mb: Option<usize>,
}

impl Web3JsonRpcConfig {
    pub fn load_config() -> Result<Web3JsonRpcConfig, config::ConfigError> {
        load_config(
            "configuration/web3_json_rpc",
            format!("{FIAMMA_COMMITTEE_PREFIX}_WEB3_JSON_RPC").as_str(),
        )
    }

    pub fn max_batch_request_size(&self) -> usize {
        self.max_batch_request_size.unwrap_or(500)
    }

    pub fn max_response_body_size(&self) -> usize {
        self.max_response_body_size_mb.unwrap_or(10) * BYTES_IN_MB
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HealthCheckConfig {
    /// Port to which the REST server is listening.
    pub port: u16,
}

impl HealthCheckConfig {
    pub fn load_config() -> Result<HealthCheckConfig, config::ConfigError> {
        load_config(
            "configuration/health_check",
            format!("{FIAMMA_COMMITTEE_PREFIX}_HEALTHCHECK").as_str(),
        )
    }

    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::new("0.0.0.0".parse().unwrap(), self.port)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BitcoinRpcConfig {
    pub http_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}

impl BitcoinRpcConfig {
    pub fn load_config() -> Result<BitcoinRpcConfig, config::ConfigError> {
        load_config(
            "configuration/bitcoin_rpc",
            format!("{FIAMMA_COMMITTEE_PREFIX}_BITCOIN_RPC").as_str(),
        )
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct FiammaRpcConfig {
    pub rpc_url: String,
}

impl FiammaRpcConfig {
    pub fn load_config() -> Result<FiammaRpcConfig, config::ConfigError> {
        load_config(
            "configuration/fiamma_rpc",
            format!("{FIAMMA_COMMITTEE_PREFIX}_FIAMMA_RPC").as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApiConfig, BitcoinRpcConfig, FiammaRpcConfig, HealthCheckConfig, Web3JsonRpcConfig,
    };
    use crate::utils::tests::EnvMutex;

    static MUTEX: EnvMutex = EnvMutex::new();

    fn default_config() -> ApiConfig {
        ApiConfig {
            web3_json_rpc: Web3JsonRpcConfig {
                http_port: 1001,
                http_url: "http://127.0.0.1:1001".to_string(),
                max_batch_request_size: Some(200),
                max_response_body_size_mb: Some(10),
            },
            healthcheck: HealthCheckConfig { port: 33001 },
            bitcoin_rpc: BitcoinRpcConfig {
                http_url: "http://127.0.0.1:18443".to_string(),
                rpc_user: "test".to_string(),
                rpc_password: "1234".to_string(),
            },
            fiamma_rpc: FiammaRpcConfig {
                rpc_url: "http://127.0.0.1:9090".to_string(),
            },
        }
    }

    #[test]
    fn test_load_api_config() {
        let mut lock = MUTEX.lock();
        let config = r#"
            FIAMMA_COMMITTEE_WEB3_JSON_RPC_HTTP_PORT=1001
            FIAMMA_COMMITTEE_WEB3_JSON_RPC_HTTP_URL=http://127.0.0.1:1001
            FIAMMA_COMMITTEE_HEALTHCHECK_PORT=33001
            FIAMMA_COMMITTEE_BITCOIN_RPC=http://127.0.0.1:18443
            FIAMMA_COMMITTEE_FIAMMA_RPC=https://testnet-grpc.fiammachain.io
        "#;
        lock.set_env(config);

        let api_config = ApiConfig::load_config().expect("failed to load api config");
        assert_eq!(api_config, default_config());
    }
}
