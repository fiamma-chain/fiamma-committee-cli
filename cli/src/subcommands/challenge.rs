use std::str::FromStr;

use bitcoin::{
    bip32::Xpriv,
    consensus::encode,
    key::TapTweak,
    sighash::{self, SighashCache},
    taproot, Address, KnownHrp, Network, OutPoint, PrivateKey, ScriptBuf, Sequence, TapLeafHash,
    TapSighashType, Transaction, TxIn, TxOut, Txid, Witness, XOnlyPublicKey,
};
use bitcoin_client::api_client::BitcoinRpcClient;
use clap::Parser;
use types::{
    challenge::{ChallengeRequest, FinishChallengeRequest},
    circuit::{CircuitInfo, CircuitType},
    constants::{CHALLENGE_FEE_AMOUNT, DUST_AMOUNT},
    file::read_vk_from_path,
};
use wallet::{provider::ProviderParams, signer::Signer, Wallet};
use web3_decl::jsonrpsee::http_client::HttpClientBuilder;

use types::constants::CHALLENGE_AMOUNT;

#[derive(Debug, Parser, Clone)]
#[clap(name = "Challenge", about = "CLI for Fiamma validator challenge")]
pub struct Challenge {
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
    #[clap(name = "start", about = "Start a challenge")]
    Start(ChallengeProof),

    #[clap(name = "status", about = "Query the started challenge status")]
    Status(ChallengeProof),

    #[clap(name = "finish", about = "Finish the challenge")]
    Finish(FillChallenge),
}

#[derive(Debug, Parser, Clone)]
pub struct ChallengeProof {
    #[clap(short, long, help = "Proof id of challenged proof")]
    proof_id: String,
    #[clap(short, long, help = "circuit's verifier key path")]
    vk_path: String,
    #[clap(short, long, help = "Circuit type")]
    circuit_type: String,
}

#[derive(Debug, Parser, Clone)]
pub struct FillChallenge {
    #[clap(short, long, help = "Proof id of challenged proof")]
    proof_id: String,

    #[clap(short, long, help = "circuit's verifier key path")]
    vk_path: String,

    #[clap(short, long, help = "Circuit type")]
    circuit_type: String,

    #[clap(short, long, help = "Assert input's transaction id")]
    txid: String,

    #[clap(short = 'o', long, help = "Assert input's vout")]
    vout: u32,

    #[clap(short = 's', long, help = "Bitcoin private key to sign assert tx")]
    private_key: String,
}

impl Challenge {
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
                let vk = read_vk_from_path(&args.vk_path)?;
                let circuit_type = CircuitType::from_str(&args.circuit_type)?;
                let circuit_info = CircuitInfo::new(&vk, circuit_type);
                let request = ChallengeRequest::new(&args.proof_id, &circuit_info.vk_hash);

                wallet
                    .start_challenge(request)
                    .await
                    .expect("Failed to start a challenge");
                println!(
                    "You have started to challenge a proof with proof_id {:?}\nNow please use `challenge status` to query the status of challenge",
                    args.proof_id
                );
            }
            Action::Status(args) => {
                let vk = read_vk_from_path(&args.vk_path)?;
                let circuit_type = CircuitType::from_str(&args.circuit_type)?;
                let circuit_info = CircuitInfo::new(&vk, circuit_type);
                let request = ChallengeRequest::new(&args.proof_id, &circuit_info.vk_hash);
                let res = wallet.challenge_status(request).await;
                println!("{}", res.unwrap());
            }
            Action::Finish(args) => {
                let vk = read_vk_from_path(&args.vk_path)?;
                let circuit_type = CircuitType::from_str(&args.circuit_type)?;
                let circuit_info = CircuitInfo::new(&vk, circuit_type);

                let request = ChallengeRequest::new(&args.proof_id, &circuit_info.vk_hash);
                let challenge_tx = wallet
                    .get_committee_challenge_tx(request)
                    .await
                    .expect("failed to get assert tx of proof");
                let multi_sig_script = wallet
                    .get_multi_sig_script_of_proof(&args.proof_id)
                    .await
                    .expect("failed to query multi sig script of proof");
                let challenge_tx = fill_challenger_tx(
                    &challenge_tx,
                    &args.private_key,
                    &args.txid,
                    args.vout,
                    multi_sig_script,
                    ctx.network,
                    &ctx.bitcoin_url(),
                    &ctx.bitcoin_username(),
                    &ctx.bitcoin_password(),
                );
                let challenge_tx_str = encode::serialize_hex(&challenge_tx);
                let request = FinishChallengeRequest::new(&args.proof_id, &challenge_tx_str);
                let res = wallet
                    .finish_challenge(request)
                    .await
                    .expect("failed to finish challenge");
                println!("You have success to finish the challenge, please check the challenge transaction ({}) status on bitcoin.", res);
            }
        }
        Ok(())
    }
}

fn fill_challenger_tx(
    raw_tx: &str,
    challenger_sk: &str,
    challenger_input_txid: &str,
    challenger_input_vout: u32,
    multi_sig_script: ScriptBuf,
    network: Network,
    bitcoin_url: &str,
    bitcoin_rpc_user: &str,
    bitcoin_rpc_password: &str,
) -> Transaction {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let multi_sig_address = Address::p2wsh(&multi_sig_script, KnownHrp::from(network));

    let private_key = if let Ok(pk) = PrivateKey::from_wif(challenger_sk) {
        pk
    } else if let Ok(pk) = Xpriv::from_str(challenger_sk) {
        pk.to_priv()
    } else {
        panic!("Invalid private key")
    };
    // let challenger_private_key =
    //     Xpriv::from_str(challenger_sk).expect("failed to recover Xpriv from private key");
    let sk = private_key.inner;
    let keypair = bitcoin::secp256k1::Keypair::from_secret_key(&secp, &sk);
    let (internal_key, _parity) = XOnlyPublicKey::from_keypair(&keypair);
    // let internal_key = Xpub::from_priv(&secp, &challenger_private_key);
    let challenger_address = Address::p2tr(&secp, internal_key, None, KnownHrp::from(network));

    // query challenger's pre tx value
    let bitcoin_rpc_client =
        BitcoinRpcClient::new(bitcoin_url, bitcoin_rpc_user, bitcoin_rpc_password)
            .expect("Failed to create bitcoin rpc client");

    let pre_txid = Txid::from_str(challenger_input_txid).expect("failed to parse tx id");
    let pre_tx = bitcoin_rpc_client
        .get_tx(pre_txid)
        .expect("tx_id is not valid");
    let utxo = pre_tx
        .tx_out(challenger_input_vout as usize)
        .expect("Invalid vout");
    let value = utxo.value;

    let out_point = OutPoint {
        txid: Txid::from_str(challenger_input_txid).unwrap(),
        vout: challenger_input_vout,
    };

    let input = TxIn {
        previous_output: out_point,
        script_sig: ScriptBuf::default(), // For a p2wpkh script_sig is empty.
        sequence: Sequence::MAX,
        witness: Witness::default(), // Filled in after signing.
    };

    let change = TxOut {
        value: value
            .checked_sub(CHALLENGE_FEE_AMOUNT.unchecked_add(CHALLENGE_AMOUNT))
            .expect("not enough btc"),
        script_pubkey: challenger_address.script_pubkey(),
    };

    let mut tx = encode::deserialize_hex::<Transaction>(raw_tx).expect("invalid raw assert tx");

    tx.input.push(input);
    tx.output.push(change);

    // // Get the sighash to sign.
    let sighash_type = TapSighashType::All;
    let mut sighasher = SighashCache::new(&mut tx);

    let input_txouts = vec![
        TxOut {
            value: DUST_AMOUNT,
            script_pubkey: multi_sig_address.script_pubkey(),
        },
        TxOut {
            value,
            script_pubkey: challenger_address.script_pubkey(),
        },
    ];
    let sighash = sighasher
        .taproot_key_spend_signature_hash(
            1,
            &sighash::Prevouts::All(input_txouts.as_slice()),
            sighash_type,
        )
        .expect("failed to create sighash");

    // Sign the sighash using the secp256k1 library (exported by rust-bitcoin).
    let keypair = bitcoin::secp256k1::Keypair::from_seckey_slice(&secp, sk.as_ref()).unwrap();
    let leaf_hash = None::<TapLeafHash>;
    let keypair = match leaf_hash {
        None => keypair.tap_tweak(&secp, None).to_inner(),
        Some(_) => keypair, // no tweak for script spend
    };

    let msg = bitcoin::secp256k1::Message::from(sighash);
    let signature = secp.sign_schnorr(&msg, &keypair);

    let final_signature = taproot::Signature {
        signature,
        sighash_type,
    };

    // Update the witness stack.
    let mut witness = Witness::new();
    witness.push(final_signature.to_vec());
    *sighasher.witness_mut(1).unwrap() = witness;

    sighasher.into_transaction().to_owned()
}
