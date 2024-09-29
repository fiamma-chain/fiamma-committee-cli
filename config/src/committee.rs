use std::str::FromStr;

use bitcoin::{bip32::Xpriv, Address, Network, PrivateKey, PublicKey, XOnlyPublicKey};
use serde::Deserialize;

use crate::{load_config, utils::tweaked_public_key, FIAMMA_COMMITTEE_PREFIX};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CommitteeConfig {
    pub public_keys: Vec<String>,
    pub network: String,
}

impl CommitteeConfig {
    pub fn load_config() -> Result<CommitteeConfig, config::ConfigError> {
        load_config(
            "configuration/committee",
            format!("{FIAMMA_COMMITTEE_PREFIX}_COMMITTEE").as_str(),
        )
    }

    pub fn get_network(&self) -> Result<Network, Box<dyn std::error::Error>> {
        match self.network.as_str() {
            "Regtest" => Ok(Network::Regtest),
            "Signet" => Ok(Network::Signet),
            "Testnet" => Ok(Network::Testnet),
            "Bitcoin" => Ok(Network::Bitcoin),
            _ => Err(Box::from("invalid network")),
        }
    }

    pub fn private_key(&self) -> Option<PrivateKey> {
        let cb_sk = std::env::var("COMMITTEE_BITCOIN_SK").ok()?;
        let private_key = if let Ok(pk) = PrivateKey::from_wif(&cb_sk) {
            pk
        } else if let Ok(pk) = Xpriv::from_str(&cb_sk) {
            pk.to_priv()
        } else {
            return None;
        };

        Some(private_key)
    }

    pub fn get_tweaked_public_key(&self) -> Option<String> {
        let public_key = self.public_keys.first()?;
        let public_key = PublicKey::from_str(public_key).ok()?;
        Some(tweaked_public_key(public_key).to_string())
    }

    pub fn get_committee_address(&self) -> Address {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let network = self.get_network().expect("failed to get network");
        let public_key = self
            .public_keys
            .first()
            .expect("failed to get committee public_key");
        let public_key =
            PublicKey::from_str(public_key).expect("failed to get committee's public key");
        let x_only = XOnlyPublicKey::from(public_key);

        Address::p2tr(&secp, x_only, None, network)
    }

    pub fn get_burn_address(&self) -> Address {
        let network = self.get_network().expect("failed to get network");
        match network {
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
            Network::Bitcoin => Address::from_str("1BitcoinEaterAddressDontSendf59kuE")
                .expect("failed to create burn address")
                .assume_checked(),
            _ => panic!("other bitcoin network not supported"),
        }
    }
}
