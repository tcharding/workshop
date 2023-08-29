// SPDX-License-Identifier: CC0-1.0

//! Demonstrate creating a transaction that spends to and from p2tr outputs.

use std::str::FromStr;

use bitcoin::hashes::Hash;
use bitcoin::key::{KeyPair, TapTweak, TweakedKeyPair, UntweakedPublicKey};
use bitcoin::locktime::absolute;
use bitcoin::secp256k1::{rand, Message, Secp256k1, SecretKey, Signing, Verification};
use bitcoin::sighash::{Prevouts, SighashCache, TapSighashType};
use bitcoin::{
    Address, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};

const DUMMY_UTXO_AMOUNT: u64 = 20_000_000;
const SPEND_AMOUNT: u64 = 5_000_000;
const CHANGE_AMOUNT: u64 = 14_999_000; // 1000 sat fee.

fn main() {
    // We need a signing secp256k1 context, if you have not seen this before just pass it in when
    // needed and otherwise ignore it.
    let secp = Secp256k1::new();

    // Get a keypair we control. In a real application these would come from a stored secret.
    let keypair = senders_keys(&secp);
    let (internal_key, _parity) = keypair.x_only_public_key();

    // Get an unspent output that is locked to the key above that we control.
    // In a real application these would come from the chain.
    let (dummy_out_point, dummy_utxo) = dummy_unspent_transaction_output(&secp, internal_key);

    // Get an address to send to.
    let address = receivers_address();

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
fn senders_keys<C: Signing>(secp: &Secp256k1<C>) -> KeyPair {
    let sk = SecretKey::new(&mut rand::thread_rng());
    KeyPair::from_secret_key(secp, &sk)
}

/// A dummy address for the receiver.
///
/// We lock the spend output to the key associated with this address.
///
/// (FWIW this is an arbitrary mainnet address from block 805222.)
fn receivers_address() -> Address {
    Address::from_str("bc1p0dq0tzg2r780hldthn5mrznmpxsxc0jux5f20fwj0z3wqxxk6fpqm7q0va")
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
fn dummy_unspent_transaction_output<C: Verification>(
    secp: &Secp256k1<C>,
    internal_key: UntweakedPublicKey,
) -> (OutPoint, TxOut) {
    let script_pubkey = ScriptBuf::new_v1_p2tr(secp, internal_key, None);

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
