use bitcoin::{consensus::encode, Address, KnownHrp, OutPoint, Transaction, TxOut};
use clap::Parser;
use transactions::disprove::DisproveTransaction;
use types::{
    constants::{CONNECTOR_C_INDEX, DUST_AMOUNT},
    register::QueryAssertTxReq,
    FinishRegisterRequest, RegisterRequest,
};
use wallet::{provider::ProviderParams, signer::Signer, Wallet};
use web3_decl::jsonrpsee::http_client::HttpClientBuilder;

use crate::subcommands::Tx;

use super::tx::CreateTx;

#[derive(Debug, Parser, Clone)]
#[clap(name = "Register", about = "CLI for Fiamma validator registration")]
pub struct Register {
    #[clap(
        short,
        long,
        help = "Network name, can be local or dev",
        default_value = "local"
    )]
    network: Option<String>,

    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Parser, Clone)]
pub enum Action {
    #[clap(
        name = "start",
        about = "Start register the validator",
        long_about = "Start register the validator.\n\nExample:\n  fcli register -n local start -v your_validator_key -t your_txid -o your_vout -s your_private_key -u your_bitcoin_rpc_user -p your_bitcoin_rpc_password"
    )]
    Start(StartRegisterInfo),

    #[clap(
        name = "finish",
        about = "Finish register the validator",
        long_about = "Finish register the validator.\n\nExample:\n  fcli register -n local start -v your_validator_key -s your_private_key -u your_bitcoin_rpc_user -p your_bitcoin_rpc_password"
    )]
    Finish(FinishRegisterInfo),
}

#[derive(Debug, Parser, Clone)]
pub struct StartRegisterInfo {
    #[clap(short, long, help = "Fiamma validator key")]
    pub validator_key: String,

    #[clap(flatten)]
    pub create_tx: CreateTx,
}

#[derive(Debug, Parser, Clone)]
pub struct FinishRegisterInfo {
    #[clap(short, long, help = "Fiamma validator key")]
    pub validator_key: String,

    #[clap(short = 's', long, help = "Bitcoin private key to sign stake tx")]
    pub private_key: String,
}

impl Register {
    pub async fn run(self) -> anyhow::Result<()> {
        let ctx = if let Some(network) = self.network {
            match network.as_str() {
                "local" => ProviderParams::local(),
                "dev" => ProviderParams::dev(),
                _ => {
                    anyhow::bail!("invalid network name")
                }
            }
        } else {
            ProviderParams::local()
        };

        let client = HttpClientBuilder::default()
            .build(ctx.http_endpoint.as_str())
            .unwrap();
        let signer = Signer::new();
        let wallet = Wallet::new(client, signer);

        match self.action {
            Action::Start(args) => {
                let aux = Tx::parse_private_key(&args.create_tx.private_key, &ctx);
                let (stake_tx, assert_tx, challenge_tx) =
                    Tx::create_presign_transactions(&args.create_tx, &ctx, &wallet, &aux).await;
                let stake_tx_hex = encode::serialize_hex(&stake_tx);
                let assert_tx_hex = encode::serialize_hex(&assert_tx);
                let challenge_tx_hex = encode::serialize_hex(&challenge_tx);

                // call register rpc
                let req = RegisterRequest::new(
                    &args.validator_key,
                    &aux.pubkey,
                    &stake_tx_hex,
                    &assert_tx_hex,
                    &challenge_tx_hex,
                );

                let register_id = wallet
                    .start_register(req)
                    .await
                    .expect("Failed to register new node");

                println!("You have submitted your registration application.\nThe registration number is {}, please wait patiently.", register_id);
            }
            Action::Finish(args) => {
                let aux = Tx::parse_private_key(&args.private_key, &ctx);

                let multi_sig_script = wallet
                    .get_multi_sig_script(&aux.pubkey)
                    .await
                    .expect("Failed to get multi_sig script");
                let multi_sig_addr = Address::p2wsh(&multi_sig_script, KnownHrp::from(ctx.network));
                let script_pubkey = multi_sig_addr.script_pubkey();

                let request = QueryAssertTxReq::new(&args.validator_key);
                let assert_tx = wallet
                    .get_committee_assert_tx(request)
                    .await
                    .expect("failed to get assert tx of validator");
                let assert_tx = encode::deserialize_hex::<Transaction>(&assert_tx)
                    .expect("failed to decode assert tx");

                let assert_tx_id = assert_tx.compute_txid();

                let out_point = vec![OutPoint {
                    txid: assert_tx_id,
                    vout: CONNECTOR_C_INDEX,
                }];

                let tx_out = vec![TxOut {
                    value: DUST_AMOUNT,
                    script_pubkey: script_pubkey.clone(),
                }];

                let input_utxos: Vec<(OutPoint, TxOut)> =
                    out_point.into_iter().zip(tx_out.into_iter()).collect();

                let disprove_tx = DisproveTransaction::new(
                    aux.private_key,
                    input_utxos,
                    multi_sig_script.clone(),
                );

                let disprove_tx_hex = encode::serialize_hex(&disprove_tx.tx);

                let request = FinishRegisterRequest::new(&args.validator_key, &disprove_tx_hex);

                let register_id = wallet
                    .finish_register(request)
                    .await
                    .expect("Failed to register new node");

                println!("You have finished your registration application.\nThe registration number is {}, please wait patiently.", register_id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register() {
        let create_tx = CreateTx {
            txid: "d54d867a330aee1500d648792ad0aaee3d9019f806e6ab514e995472e2696e15".to_string(),
            vout: 0,
            private_key: "tprv8jzau9CfsdkXMzqWFWSgu7f4z1vRk53yiqYqByfoakSLNFQ4bBuTsrUDLXtKHTPZhp161h49vEJr2zwN92G7ZHLZMFvome2U8GcAqDzVRhW".to_string(),
        };
        let register = StartRegisterInfo {
            validator_key: "fiammavaloper19fldhw0awjv2ag7dz0lr3d4qmnfkxz69vukt7x".to_string(),
            create_tx,
        };
        let register = Register {
            action: Action::Start(register),
            network: Some("local".to_string()),
        };

        let _ = register.run().await;
    }
}
