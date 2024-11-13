use bitcoin::{consensus, Network, Transaction};
use reqwest::blocking::Client;
pub struct MempoolClient {
    http_client: Client,
    base_url: String,
}

impl MempoolClient {
    pub fn new(network: Network) -> Self {
        let base_url = match network {
            Network::Bitcoin => "https://mempool.space/api",
            Network::Testnet => "https://mempool.space/testnet/api",
            Network::Signet => "https://mempool.space/signet/api",
            _ => panic!("Network not supported"),
        }
        .to_string();

        Self {
            http_client: Client::new(),
            base_url,
        }
    }

    pub fn get_tx(&self, txid: &str) -> Result<Transaction, reqwest::Error> {
        let url = format!("{}/tx/{}", self.base_url, txid);
        let response = self.http_client.get(&url).send()?.bytes()?;

        Ok(consensus::deserialize(&response[..]).unwrap())
    }

    pub fn broadcast_transaction(&self, tx: &Transaction) -> Result<String, reqwest::Error> {
        let url = format!("{}/tx", self.base_url);
        let tx_hex = consensus::serialize(tx);

        let response = self.http_client.post(&url).body(tx_hex).send()?.text()?;

        Ok(response) // Returns txid
    }
}
