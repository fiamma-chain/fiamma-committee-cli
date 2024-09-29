pub use bitcoincore_rpc::json::GetRawTransactionResult;
use bitcoincore_rpc::{Auth, Client, Result, RpcApi};

#[derive(Debug)]
pub struct BitcoinRpcClient {
    client: Client,
}

impl BitcoinRpcClient {
    pub fn new(url: &str, user: &str, password: &str) -> Result<Self> {
        let client = Client::new(url, Auth::UserPass(user.to_string(), password.to_string()))?;
        Ok(Self { client })
    }

    pub fn post_tx(&self, tx: String) -> Result<bitcoin::Txid> {
        self.client.send_raw_transaction(tx)
    }

    pub fn get_tx(&self, tx_id: bitcoin::Txid) -> Result<bitcoin::Transaction> {
        self.client.get_raw_transaction(&tx_id, None)
    }

    pub fn get_tx_info(&self, tx_id: bitcoin::Txid) -> Result<GetRawTransactionResult> {
        self.client.get_raw_transaction_info(&tx_id, None)
    }

    pub fn get_block_count(&self) -> Result<u64> {
        self.client.get_block_count()
    }
}

#[cfg(test)]
mod tests {
    use bitcoincore_rpc::{Auth, Client, RpcApi};

    #[test]
    fn test_rpc() {
        let url = "http://127.0.0.1:18443";
        let user = "test".to_string();
        let pass = "1234".to_string();
        let rpc = Client::new(url, Auth::UserPass(user, pass)).unwrap();
        let _blockchain_info = rpc.get_blockchain_info();
        // println!("blockchain_info: {:?}", blockchain_info);

        // 1bebdeb5523f2efb07ebc1e4b636e83071b66cce83c14b7b5cec394bc6a1b178

        // let tx_id =
        //     Txid::from_str("a3c6e68ea08cb885f8b6cace333daca6102f4498a0996c6c00ac747726df17dd")
        //         .unwrap();
        // // let transaction = rpc.get_raw_transaction(&tx_id, None).unwrap();
        // let transaction_info = rpc.get_raw_transaction_info(&tx_id, None).unwrap();
        // println!("{:?}", transaction_info);

        let raw_signed_tx = "0200000000010113953490481ce4c7d8a7df2b3a5544d382a7db72904a5b9bcc2d9645bda3ab9c0000000000ffffffff028096980000000000220020748d118052d6e418922165b03a3191cb70ef216aa65428d6ca8951d20e78bdda98576d2901000000225120be27fa8b1f5278faf82cab8da23e8761f8f9bd5d5ebebbb37e0e12a70d92dd160141759ea221004211674874af3c603316aab7e7ff1e4c8217f224c4104b1ae353e64a00de10fdd44a9f3adc877ec10e2decbfaa80005c4951c433010012d9a50aab0100000000";

        let resp = rpc.send_raw_transaction(raw_signed_tx);
        println!("resp: {:?}", resp);
    }
}
