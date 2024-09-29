use std::str::FromStr;

use bitcoin::{Address, Network};

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
            http_endpoint: "http://54.65.137.66:33000".to_string(),
        }
    }

    pub fn dev_regtest() -> Self {
        Self {
            network: Network::Regtest,
            http_endpoint: "http://54.65.137.66:33000".to_string(),
        }
    }

    pub fn is_dev(&self) -> bool {
        self.http_endpoint == "http://54.65.137.66:33000"
    }

    pub fn get_burn_address(&self) -> Address {
        match self.network {
            Network::Regtest => Address::from_str(
                "bcrt1pmdx8nnpllj3x750zzfqmjvedv34swuka06vda8qau6csnyx2hq9s6p89qf",
            )
            .expect("failed to create burn address")
            .assume_checked(),
            Network::Signet => {
                Address::from_str("tb1px3zjhc60v2y7p8a2nkv2zymnwr0wx4pwurgktc9ly5yfu3vk6fjq05ey7n")
                    .expect("failed to create burn address")
                    .assume_checked()
            }
            _ => panic!("other bitcoin network not supported"),
        }
    }

    pub fn bitcoin_url(&self) -> String {
        match self.network {
            Network::Regtest => {
                if self.is_dev() {
                    "http://54.65.137.66:18443".to_string()
                } else {
                    "http://127.0.0.1:18443".to_string()
                }
            }
            Network::Signet => "http://54.65.137.66:38332".to_string(),
            _ => panic!("other bitcoin network not supported"),
        }
    }

    pub fn bitcoin_username(&self) -> String {
        match self.network {
            Network::Regtest => "test".to_string(),
            Network::Signet => "fiamma".to_string(),
            _ => panic!("other bitcoin network not supported"),
        }
    }

    pub fn bitcoin_password(&self) -> String {
        match self.network {
            Network::Regtest => "1234".to_string(),
            Network::Signet => "fiamma".to_string(),
            _ => panic!("other bitcoin network not supported"),
        }
    }
}
