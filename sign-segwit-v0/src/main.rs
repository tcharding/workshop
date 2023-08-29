// SPDX-License-Identifier: CC0-1.0

//! Sign a transaction that spends an p2wpkh unspent output.

use std::str::FromStr;

use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute;
use bitcoin::secp256k1::{rand, Message, Secp256k1, SecretKey, Signing};
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::{
    Address, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, WPubkeyHash,
    Witness,
};

const DUMMY_UTXO_AMOUNT: u64 = 20_000_000;
const SPEND_AMOUNT: u64 = 5_000_000;
const CHANGE_AMOUNT: u64 = 14_999_000; // 1000 sat fee.

fn main() {
    // We need a signing secp256k1 context, if you have not seen this before just pass it in when
    // needed and otherwise ignore it.
    let secp = Secp256k1::new();

    // Get a secret key we control and the pubkeyhash of the associated pubkey.
    // In a real application these would come from a stored secret.
    let (sk, wpkh) = senders_keys(&secp);

    // Get an address to send to.
    let address = receivers_address();

    // Get an unspent output that is locked to the key above that we control.
    // In a real application these would come from the chain.
    let (dummy_out_point, dummy_utxo) = dummy_unspent_transaction_output(&wpkh);

    // The script code required to spend a p2wpkh output.
    let script_code = todo!();

    // The input for the transaction we are constructing.
    let input = todo!();

    // The spend output is locked to a key controlled by the receiver.
    let spend = todo!();

    // The change output is locked to a key controlled by us.
    let change = todo!();

    // The transaction we want to sign and broadcast.
    let mut unsigned_tx = todo!();

    //
    // TODO: Sign the unsigned transaction.
    //
}

/// An example of keys controlled by the transaction sender.
///
/// In a real application these would be actual secrets.
fn senders_keys<C: Signing>(secp: &Secp256k1<C>) -> (SecretKey, WPubkeyHash) {
    let sk = SecretKey::new(&mut rand::thread_rng());
    let pk = bitcoin::PublicKey::new(sk.public_key(secp));
    let wpkh = pk.wpubkey_hash().expect("key is compressed");

    (sk, wpkh)
}

/// A dummy address for the receiver.
///
/// We lock the spend output to the key associated with this address.
///
/// (FWIW this is an arbitrary mainnet address.)
fn receivers_address() -> Address {
    Address::from_str("bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf")
        .expect("a valid address")
        .require_network(Network::Bitcoin)
        .expect("valid address for mainnet")
}

/// Creates a p2wpkh output locked to the key associated with `wpkh`.
///
/// An utxo is described by the `OutPoint` (txid and index within the transaction that it was
/// created). Using the out point one can get the transaction by `txid` and using the `vout` get the
/// transaction value and script pubkey (`TxOut`) of the utxo.
///
/// This output is locked to keys that we control, in a real application this would be a valid
/// output taken from a transaction that appears in the chain.
fn dummy_unspent_transaction_output(wpkh: &WPubkeyHash) -> (OutPoint, TxOut) {
    let script_pubkey = ScriptBuf::new_v0_p2wpkh(wpkh);

    let out_point = OutPoint {
        txid: Txid::all_zeros(), // Obviously invalid.
        vout: 0,
    };

    let utxo = TxOut {
        value: DUMMY_UTXO_AMOUNT,
        script_pubkey,
    };

    (out_point, utxo)
}
