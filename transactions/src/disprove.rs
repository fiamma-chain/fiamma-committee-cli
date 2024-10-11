use bitcoin::{OutPoint, PrivateKey, ScriptBuf, Transaction, TxOut};

use crate::create_tx_with_single_signature;

pub struct DisproveTransaction {
    pub tx: Transaction,
}

impl DisproveTransaction {
    pub fn new(
        private_key: &PrivateKey,
        input_utxo: Vec<(OutPoint, TxOut)>,
        witness_script: ScriptBuf,
    ) -> Self {
        let tx = create_tx_with_single_signature(private_key.inner, input_utxo, witness_script);
        Self { tx }
    }
}
