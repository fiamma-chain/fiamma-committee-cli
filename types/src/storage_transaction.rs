use crate::transaction::{FiammaTransaction, TransactionType};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageFiammaTransaction {
    pub tx_id: Vec<u8>,
    pub tx_type: i64,
    pub data: Vec<u8>,
    pub ext_info: Option<serde_json::Value>,
    pub register_id: i64,
}

impl From<StorageFiammaTransaction> for FiammaTransaction {
    fn from(tx: StorageFiammaTransaction) -> Self {
        let tx_id = String::from_utf8(tx.tx_id.clone()).unwrap_or_else(|e| {
            panic!(
                "Transaction ID {:#?} is invalid for transaction. Error: {}",
                tx.tx_id, e
            )
        });
        let tx_type = TransactionType::from(tx.tx_type as i32);
        let data = String::from_utf8(tx.data).unwrap_or_else(|e| {
            panic!("Transaction ID {:#?} has invalid data. Error: {}", tx_id, e)
        });
        let ext_info = tx.ext_info.and_then(|v| v.as_str().map(|s| s.to_string()));
        Self {
            tx_id,
            tx_type,
            data,
            ext_info,
            register_id: tx.register_id as u32,
        }
    }
}
