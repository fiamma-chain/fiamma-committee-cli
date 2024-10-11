use std::{collections::BTreeMap, str::FromStr};

use anyhow::Context;
use bitcoin::{
    absolute, consensus,
    key::TapTweak,
    psbt::{self, Input, PsbtSighashType},
    secp256k1::{self, Message, SecretKey},
    sighash::{self, SighashCache},
    taproot, transaction, Amount, EcdsaSighashType, OutPoint, PrivateKey, Psbt,
    ScriptBuf, SegwitV0Sighash, Sequence, TapLeafHash, TapSighash, TapSighashType, Transaction,
    TxIn, TxOut, Witness, XOnlyPublicKey,
};
use types::P2trUtxo;

pub mod assert;
pub mod challenge;
pub mod disprove;
pub mod stake;
pub mod types;

fn generate_bip86_key_spend_tx(
    secp: &secp256k1::Secp256k1<secp256k1::All>,
    private_key: PrivateKey,
    input_utxo: P2trUtxo,
    outputs: Vec<TxOut>,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let from_amount = input_utxo.amount_in_sats;
    let input_pubkey = XOnlyPublicKey::from_str(input_utxo.pubkey)?;

    // CREATOR + UPDATER
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: input_utxo.txid.parse()?,
                vout: input_utxo.vout,
            },
            script_sig: ScriptBuf::new(),
            sequence: bitcoin::Sequence::MAX, // Ignore nSequence.
            witness: Witness::default(),
        }],
        output: outputs,
    };
    let mut psbt = Psbt::from_unsigned_tx(tx)?;

    let mut input = Input {
        witness_utxo: {
            let script_pubkey = ScriptBuf::from_hex(input_utxo.script_pubkey)
                .expect("failed to parse input utxo scriptPubkey");
            Some(TxOut {
                value: from_amount,
                script_pubkey,
            })
        },
        ..Default::default()
    };
    let ty = PsbtSighashType::from_str("SIGHASH_ALL")?;
    input.sighash_type = Some(ty);
    input.tap_internal_key = Some(input_pubkey);
    psbt.inputs = vec![input];

    // The `Prevouts::All` array is used to create the sighash to sign for each input in the
    // `psbt.inputs` array, as such it must be the same length and in the same order as the inputs.
    let mut input_txouts = Vec::<TxOut>::new();
    for input in [&input_utxo].iter() {
        input_txouts.push(TxOut {
            value: input.amount_in_sats,
            script_pubkey: ScriptBuf::from_hex(input.script_pubkey)?,
        });
    }

    // SIGNER
    let unsigned_tx = psbt.unsigned_tx.clone();
    psbt.inputs
        .iter_mut()
        .enumerate()
        .try_for_each::<_, Result<(), Box<dyn std::error::Error>>>(|(vout, input)| {
            let sighash_type = input
                .sighash_type
                .and_then(|psbt_sighash_type| psbt_sighash_type.taproot_hash_ty().ok())
                .unwrap_or(TapSighashType::All);
            let hash = SighashCache::new(&unsigned_tx).taproot_key_spend_signature_hash(
                vout,
                &sighash::Prevouts::All(input_txouts.as_slice()),
                sighash_type,
            )?;

            let secret_key: secp256k1::SecretKey = private_key.inner;
            sign_psbt_taproot(
                &secret_key,
                input.tap_internal_key.unwrap(),
                None,
                input,
                hash,
                sighash_type,
                secp,
            );

            Ok(())
        })?;

    // FINALIZER
    psbt.inputs.iter_mut().for_each(|input| {
        let mut script_witness: Witness = Witness::new();
        script_witness.push(input.tap_key_sig.unwrap().to_vec());
        input.final_script_witness = Some(script_witness);

        // Clear all the data fields as per the spec.
        input.partial_sigs = BTreeMap::new();
        input.sighash_type = None;
        input.redeem_script = None;
        input.witness_script = None;
        input.bip32_derivation = BTreeMap::new();
    });

    // EXTRACTOR
    let tx = psbt.extract_tx_unchecked_fee_rate();
    tx.verify(|_| {
        Some(TxOut {
            value: from_amount,
            script_pubkey: ScriptBuf::from_hex(input_utxo.script_pubkey).unwrap(),
        })
    })
    .expect("failed to verify transaction");

    Ok(tx)
}

fn create_tx_with_single_signature(
    sk: SecretKey,
    pre_outs: Vec<(OutPoint, TxOut)>,
    witness_script: ScriptBuf,
) -> Transaction {
    let secp = secp256k1::Secp256k1::new();

    let inputs = pre_outs
        .iter()
        .map(|pre_out| TxIn {
            previous_output: pre_out.0,
            script_sig: ScriptBuf::default(), // For a p2wsh script_sig is empty.
            sequence: Sequence::MAX,
            witness: Witness::default(), // Filled in after signing.
        })
        .collect();

    let mut unsigned_tx = Transaction {
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
        input: inputs,
        output: vec![],
    };

    // Get the sighash to sign.
    let sighash_type = EcdsaSighashType::NonePlusAnyoneCanPay;
    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    for (idx, pre_out) in pre_outs.iter().enumerate() {
        let sighash = sighasher
            .p2wsh_signature_hash(idx, &witness_script, pre_out.1.value, sighash_type)
            .expect("failed to create sighash");
        // Sign the sighash using the secp256k1 library (exported by rust-bitcoin).
        let msg = Message::from(sighash);
        let signature = secp.sign_ecdsa(&msg, &sk);

        // Update the witness stack.
        let signature = bitcoin::ecdsa::Signature {
            signature,
            sighash_type,
        };

        secp.verify_ecdsa(&msg, &signature.signature, &sk.public_key(&secp))
            .unwrap();

        let mut witness = Witness::new();
        witness.push_ecdsa_signature(&signature);
        witness.push(witness_script.clone());
        *sighasher.witness_mut(idx).unwrap() = witness;
    }

    sighasher.into_transaction().to_owned()
}

pub fn compute_sighash_p2wsh(
    raw_tx: &[u8],
    inp_idx: usize,
    value: Amount,
    sighash_type: EcdsaSighashType,
    witness_script: ScriptBuf,
) -> anyhow::Result<SegwitV0Sighash> {
    let tx: Transaction = consensus::deserialize(raw_tx).unwrap();

    let mut cache = sighash::SighashCache::new(&tx);
    cache
        .p2wsh_signature_hash(inp_idx, &witness_script, value, sighash_type)
        .context("failed to compute sighash")
}

pub fn sign_psbt_taproot(
    secret_key: &secp256k1::SecretKey,
    pubkey: XOnlyPublicKey,
    leaf_hash: Option<TapLeafHash>,
    psbt_input: &mut psbt::Input,
    hash: TapSighash,
    sighash_type: TapSighashType,
    secp: &secp256k1::Secp256k1<secp256k1::All>,
) {
    let keypair = secp256k1::Keypair::from_seckey_slice(secp, secret_key.as_ref()).unwrap();
    let keypair = match leaf_hash {
        None => keypair
            .tap_tweak(secp, psbt_input.tap_merkle_root)
            .to_inner(),
        Some(_) => keypair, // no tweak for script spend
    };

    let msg = secp256k1::Message::from(hash);
    let signature = secp.sign_schnorr(&msg, &keypair);

    let final_signature = taproot::Signature {
        signature,
        sighash_type,
    };

    if let Some(lh) = leaf_hash {
        psbt_input
            .tap_script_sigs
            .insert((pubkey, lh), final_signature);
    } else {
        psbt_input.tap_key_sig = Some(final_signature);
    }
}
