use bitcoin::{secp256k1::Secp256k1, PrivateKey, Transaction, TxOut};

use crate::{generate_bip86_key_spend_tx, types::P2trUtxo};

pub struct StakeTransaction {
    tx: Transaction,
}

impl StakeTransaction {
    pub fn new(private_key: PrivateKey, input_utxo: P2trUtxo, outputs: Vec<TxOut>) -> Self {
        let secp = Secp256k1::new();
        let tx = generate_bip86_key_spend_tx(&secp, private_key, input_utxo, outputs)
            .expect("failed to generate stake tx");

        StakeTransaction { tx }
    }

    pub fn extract_tx(&self) -> Transaction {
        self.tx.clone()
    }
}
