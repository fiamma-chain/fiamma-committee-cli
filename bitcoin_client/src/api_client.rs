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
        let url = format!("{}/tx/{}/hex", self.base_url, txid);
        let response = self.http_client.get(&url).send()?;

        let tx_bytes = hex::decode(response.text()?.trim()).unwrap();
        Ok(consensus::deserialize(&tx_bytes).unwrap())
    }
}
