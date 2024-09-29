use bitcoin::Amount;

#[derive(Debug, Clone, Copy)]
pub struct P2trUtxo<'a> {
    pub txid: &'a str,
    pub vout: u32,
    pub script_pubkey: &'a str,
    pub pubkey: &'a str,
    pub amount_in_sats: Amount,
}
