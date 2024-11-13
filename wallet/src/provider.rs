use bitcoin::Network;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderParams {
    pub network: Network,
    pub http_endpoint: String,
}

impl ProviderParams {
    pub fn new(network: Network, http_endpoint: String) -> Self {
        Self {
            network,
            http_endpoint,
        }
    }

    pub fn local() -> Self {
        Self {
            network: Network::Regtest,
            http_endpoint: "http://127.0.0.1:33000".to_string(),
        }
    }

    pub fn dev() -> Self {
        Self {
            network: Network::Signet,
            http_endpoint: "http://54.65.75.57:33000".to_string(),
        }
    }

    pub fn testnet() -> Self {
        Self {
            network: Network::Signet,
            http_endpoint: "https://testnet-committee.fiammachain.io".to_string(),
        }
    }

    pub fn dev_regtest() -> Self {
        Self {
            network: Network::Regtest,
            http_endpoint: "http://54.65.75.57:33000".to_string(),
        }
    }

    pub fn is_dev(&self) -> bool {
        self.http_endpoint == "http://54.65.75.57:33000"
    }
}
