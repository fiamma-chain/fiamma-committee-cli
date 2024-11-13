use std::str::FromStr;

use bitcoin::{
    bip32::Xpriv, consensus::encode, secp256k1, Address, KnownHrp, OutPoint, PrivateKey, PublicKey,
    ScriptBuf, Transaction, TxOut, Txid, XOnlyPublicKey,
};
use bitcoin_client::api_client::MempoolClient;
use clap::Parser;
use transactions::{
    assert::AssertTransaction, challenge::ChallengeTransaction, stake::StakeTransaction,
    types::P2trUtxo,
};
use types::constants::{
    ASSERT_FEE_AMOUNT, CONNECTOR_A_INDEX, CONNECTOR_B_INDEX, DUST_AMOUNT, STAKE_AMOUNT,
    STAKE_FEE_AMOUNT, STAKE_VALUE_INDEX,
};
use wallet::{provider::ProviderParams, signer::Signer, Wallet};
use web3_decl::jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

#[derive(Debug, Parser, Clone)]
#[clap(
    name = "Tx",
    about = "CLI for Fiamma validator registration and staking related transaction"
)]
pub struct Tx {
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
        name = "address",
        about = "Print the stake address",
        long_about = "Print the stake address by public key.\n\nExample:\n  fcli tx -n local address -p your_public_key"
    )]
    GetStakeAddress(GetStakeAddress),

    #[clap(
        name = "stake",
        about = "Create a stake transaction",
        long_about = "Create a stake transaction.\n\nExample:\n  fcli tx -n local stake -t your_txid -o your_vout -s your_private_key -u your_bitcoin_rpc_user -p your_bitcoin_rpc_password"
    )]
    CreateStakeTx(CreateTx),

    #[clap(
        name = "assert",
        about = "Create an assert transaction",
        long_about = "Create an assert transaction.\n\nExample:\n  fcli tx -n local assert -t your_txid -o your_vout -s your_private_key -u your_bitcoin_rpc_user -p your_bitcoin_rpc_password"
    )]
    CreateAssertTx(CreateTx),
}

#[derive(Debug, Parser, Clone)]
pub struct GetStakeAddress {
    #[clap(short, long, help = "Public key of bitcoin keypair")]
    public_key: String,
}

#[derive(Debug, Parser, Clone)]
pub struct CreateTx {
    #[clap(short, long, help = "Stake input's transaction id")]
    pub txid: String,

    #[clap(short = 'o', long, help = "Stake input's vout")]
    pub vout: u32,

    #[clap(short = 's', long, help = "Bitcoin private key to sign stake tx")]
    pub private_key: String,
}

#[derive(Debug, Parser, Clone)]
pub struct Register {
    #[clap(short, long, help = "Fiamma validator key")]
    validator_key: String,

    #[clap(flatten)]
    create_tx: CreateTx,
}

pub struct Auxiliary {
    pub private_key: PrivateKey,
    pub pubkey: String,
    pub internal_x_only_pubkey: String,
    pub script_pk: ScriptBuf,
}

impl Tx {
    pub async fn run(self) -> anyhow::Result<()> {
        let ctx = if let Some(network) = self.network {
            match network.as_str() {
                "local" => ProviderParams::local(),
                "dev" => ProviderParams::dev(),
                "testnet" => ProviderParams::testnet(),
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
            Action::GetStakeAddress(args) => {
                let script = wallet.get_multi_sig_script(&args.public_key).await.unwrap();
                println!("Multi Sig Script:\n{script}");
            }
            Action::CreateStakeTx(args) => {
                let aux = Self::parse_private_key(&args.private_key, &ctx);
                let stake_tx = Self::create_stake_tx(&args, &ctx, &wallet, &aux).await;
                let stake_tx_hex = encode::serialize_hex(&stake_tx.0);
                println!("Stake tx:\n{:?}", stake_tx_hex);
            }
            Action::CreateAssertTx(args) => {
                let aux = Self::parse_private_key(&args.private_key, &ctx);
                let (_, assert_tx, _) =
                    Self::create_presign_transactions(&args, &ctx, &wallet, &aux).await;
                let assert_tx_hex = encode::serialize_hex(&assert_tx);
                println!("Assert tx:\n{:?}", assert_tx_hex);
            }
        }
        Ok(())
    }

    pub fn parse_private_key(private_key: &str, ctx: &ProviderParams) -> Auxiliary {
        let secp = secp256k1::Secp256k1::new();
        let private_key = if let Ok(pk) = PrivateKey::from_wif(private_key) {
            pk
        } else if let Ok(pk) = Xpriv::from_str(private_key) {
            pk.to_priv()
        } else {
            panic!("Invalid private key")
        };
        // let sk = Xpriv::from_str(private_key).expect("Invalid private key");
        let pubkey = PublicKey::from_private_key(&secp, &private_key).to_string();
        // let internal_key = Xpub::from_priv(&secp, &sk);
        let secret_key = private_key.inner;
        let keypair = secp256k1::Keypair::from_secret_key(&secp, &secret_key);
        let (internal_key, _parity) = XOnlyPublicKey::from_keypair(&keypair);
        let address = Address::p2tr(&secp, internal_key, None, KnownHrp::from(ctx.network));
        let internal_x_only_pubkey = internal_key.to_string();
        let script_pk = address.script_pubkey();
        Auxiliary {
            private_key,
            pubkey,
            internal_x_only_pubkey,
            script_pk,
        }
    }

    async fn create_stake_tx(
        args: &CreateTx,
        ctx: &ProviderParams,
        wallet: &Wallet<HttpClient>,
        aux: &Auxiliary,
    ) -> (bitcoin::Transaction, ScriptBuf, ScriptBuf) {
        let Auxiliary {
            private_key,
            pubkey,
            internal_x_only_pubkey,
            script_pk,
        } = aux;
        let txid = Txid::from_str(&args.txid).expect("Invalid txid");
        let vout = args.vout;

        let bitcoin_rpc_client = MempoolClient::new(ctx.network);
        let pre_tx = bitcoin_rpc_client
            .get_tx(&txid.to_string())
            .expect("tx_id is not valid");
        let utxo = pre_tx.tx_out(vout as usize).expect("Invalid vout");

        // generate stake tx
        let multi_sig_script = wallet
            .get_multi_sig_script(pubkey)
            .await
            .expect("Failed to get multi_sig script");
        let multi_sig_addr = Address::p2wsh(&multi_sig_script, KnownHrp::from(ctx.network));
        let script_pubkey = multi_sig_addr.script_pubkey();

        let input_utxo = P2trUtxo {
            txid: &txid.to_string(),
            vout,
            script_pubkey: &utxo.script_pubkey.to_hex_string(),
            pubkey: &internal_x_only_pubkey.to_string(),
            amount_in_sats: utxo.value,
        };

        let spent = TxOut {
            value: STAKE_AMOUNT.unchecked_add(ASSERT_FEE_AMOUNT),
            script_pubkey: script_pubkey.clone(),
        };

        // TODO: add timelock
        let unstake_timelock = TxOut {
            value: DUST_AMOUNT,
            script_pubkey: script_pubkey.clone(),
        };

        let connector_a = TxOut {
            value: DUST_AMOUNT,
            script_pubkey: script_pubkey.clone(),
        };

        let connector_b = TxOut {
            value: DUST_AMOUNT,
            script_pubkey: script_pubkey.clone(),
        };

        let change = TxOut {
            value: input_utxo
                .amount_in_sats
                .checked_sub(
                    STAKE_FEE_AMOUNT
                        .unchecked_add(STAKE_AMOUNT)
                        .unchecked_add(ASSERT_FEE_AMOUNT)
                        .unchecked_add(DUST_AMOUNT * 3),
                )
                .expect("Not enough btc"),
            script_pubkey: script_pk.clone(),
        };

        let stake_tx = StakeTransaction::new(
            *private_key,
            input_utxo,
            vec![spent, unstake_timelock, connector_a, connector_b, change],
        );
        let stake_tx = stake_tx.extract_tx();
        (stake_tx, script_pubkey, multi_sig_script)
    }

    pub async fn create_presign_transactions(
        args: &CreateTx,
        ctx: &ProviderParams,
        wallet: &Wallet<HttpClient>,
        aux: &Auxiliary,
    ) -> (Transaction, Transaction, Transaction) {
        let (stake_tx, script_pubkey, multi_sig_script) =
            Self::create_stake_tx(args, ctx, wallet, aux).await;

        let stake_txid = stake_tx.compute_txid();

        let out_point = vec![
            OutPoint {
                txid: stake_txid,
                vout: STAKE_VALUE_INDEX,
            },
            OutPoint {
                txid: stake_txid,
                vout: CONNECTOR_B_INDEX,
            },
        ];
        let tx_out = vec![
            TxOut {
                value: STAKE_AMOUNT.unchecked_add(ASSERT_FEE_AMOUNT),
                script_pubkey: script_pubkey.clone(),
            },
            TxOut {
                value: DUST_AMOUNT,
                script_pubkey: script_pubkey.clone(),
            },
        ];
        let input_utxos: Vec<(OutPoint, TxOut)> =
            out_point.into_iter().zip(tx_out.into_iter()).collect();
        let assert_tx =
            AssertTransaction::new(aux.private_key, input_utxos, multi_sig_script.clone());

        let out_point = vec![OutPoint {
            txid: stake_txid,
            vout: CONNECTOR_A_INDEX,
        }];

        let tx_out = vec![TxOut {
            value: DUST_AMOUNT,
            script_pubkey: script_pubkey.clone(),
        }];

        let input_utxos: Vec<(OutPoint, TxOut)> =
            out_point.into_iter().zip(tx_out.into_iter()).collect();

        let challenge_tx =
            ChallengeTransaction::new(aux.private_key, input_utxos, multi_sig_script.clone());

        (stake_tx, assert_tx.tx, challenge_tx.tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_stake_tx() {
        let create_stake_tx = CreateTx {
            txid: "d54d867a330aee1500d648792ad0aaee3d9019f806e6ab514e995472e2696e15".to_string(),
            vout: 0,
            private_key: "tprv8jzau9CfsdkXMzqWFWSgu7f4z1vRk53yiqYqByfoakSLNFQ4bBuTsrUDLXtKHTPZhp161h49vEJr2zwN92G7ZHLZMFvome2U8GcAqDzVRhW".to_string(),
        };
        let tx = Tx {
            action: Action::CreateStakeTx(create_stake_tx),
            network: Some("local".to_string()),
        };

        let _ = tx.run().await;
    }

    #[tokio::test]
    async fn test_create_assert_tx() {
        let create_assert_tx = CreateTx {
            txid: "d54d867a330aee1500d648792ad0aaee3d9019f806e6ab514e995472e2696e15".to_string(),
            vout: 0,
            private_key: "tprv8jzau9CfsdkXMzqWFWSgu7f4z1vRk53yiqYqByfoakSLNFQ4bBuTsrUDLXtKHTPZhp161h49vEJr2zwN92G7ZHLZMFvome2U8GcAqDzVRhW".to_string(),
        };
        let tx = Tx {
            action: Action::CreateAssertTx(create_assert_tx),
            network: Some("local".to_string()),
        };

        let _ = tx.run().await;
    }

    #[tokio::test]
    async fn test_get_stake_address() {
        let get_stake_address = GetStakeAddress {
            public_key: "02ff12471208c14bd580709cb2358d98975247d8765f92bc25eab3b2763ed605f8"
                .to_string(),
        };

        let tx = Tx {
            action: Action::GetStakeAddress(get_stake_address),
            network: None,
        };

        let _ = tx.run().await;
    }

    // cargo run --bin fcli tx register -t d54d867a330aee1500d648792ad0aaee3d9019f806e6ab514e995472e2696e15 -o 0 -s tprv8jzau9CfsdkXMzqWFWSgu7f4z1vRk53yiqYqByfoakSLNFQ4bBuTsrUDLXtKHTPZhp161h49vEJr2zwN92G7ZHLZMFvome2U8GcAqDzVRhW -u test -p 1234 -v fiammavaloper19fldhw0awjv2ag7dz0lr3d4qmnfkxz69vukt7x
}
