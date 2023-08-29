//! Insanely minimal Bitcoin wallet intended for demonstration of [Rust Bitcoin] ecosystem
//!
//! [Rust Bitcoin]: https://rust-bitcoin.org

#![allow(unused_imports)]
#![allow(dead_code)]

use std::convert::TryInto;

use anyhow::{anyhow, bail, Context, Result};
use bitcoin::key::TapTweak;
use bitcoin::{
    transaction, Address, Amount, FeeRate, Network, OutPoint, PrivateKey, Sequence, Transaction,
    TxIn, TxOut, Witness,
};
use bitcoincore_rpc::{Client, RpcApi};

mod config;
mod db;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next().ok_or_else(|| anyhow!("program name missing"))?;
    match args.next() {
        None => {
            println!("Command missing\n\n");
            help()
        }
        Some(command) => match &*command {
            "scan" => scan(),
            "address" => address(),
            "balance" => balance(),
            "send" => send(args),
            "help" | "--help" | "-h" => help(),
            _ => bail!("Unknown command: `{}`", command),
        },
    }
}

/// Prints an address associated with the private key loaded from file.
///
/// In a production wallet one would never reuse a single address like this but for demonstration
/// purposes it will suffice.
///
/// You can use a taproot address if you would like to play with taproot spends or alternatively you
/// can use a segwit v0 address. Note that the PSBT signing APIs are slightly different for each.
fn address() -> Result<()> {
    let address = get_address()?;
    println!("{}", address);
    Ok(())
}

fn get_address() -> Result<Address> {
    let private_key = load_private_key()?;
    let pub_key = private_key
        .inner
        .x_only_public_key(&**secp256k1::SECP256K1)
        .0;

    Ok(Address::p2tr(
        &secp256k1::SECP256K1,
        pub_key,
        None,
        Network::Regtest,
    ))
}

/// Scans the Bitcoin blockchain.
///
/// Requests blocks from `bitcoind`, starting at the current block height (`db.get_last_height`) and
/// stores relevant transaction information in the database.
///
/// Call this each time you use `bitcoin-cli generatetoaddress` to mine coins to your address.
fn scan() -> Result<()> {
    let conf = config::load()?;
    let connection = bitcoincore_rpc::Client::new(&conf.bitcoind_uri, conf.bitcoind_auth)
        .context("failed to connect to bitcoind")?;
    let current_height = connection
        .get_block_count()
        .context("Failed to get block count")?;
    let mut db = db::Db::open()?;
    let last_height = db.get_last_height()?;
    let script_pubkey = get_address()?.script_pubkey();
    // we need to move txid below but not `script_pubkey`
    let script_pubkey = &script_pubkey;
    let mut block_count = 0;
    let mut tx_count = 0;
    let mut txos = 0;
    let mut total_amount = 0;
    let txos_iter = ((last_height + 1)..=current_height)
        .flat_map(|height| {
            let block = connection
                .get_block_hash(height)
                .context("Failed to get block hash")
                .and_then(|block_hash| {
                    connection
                        .get_block(&block_hash)
                        .context("Failed to get block hash")
                });
            match block {
                Ok(block) => {
                    block_count += 1;
                    either::Left(block.txdata.into_iter().map(Ok))
                }
                Err(error) => either::Right(std::iter::once(Err(error))),
            }
        })
        .flat_map(|transaction| match transaction {
            Ok(transaction) => {
                tx_count += 1;
                let txid = transaction.txid();
                let iter = transaction
                    .output
                    .into_iter()
                    .enumerate()
                    .map(move |(i, txout)| Ok((txid, i, txout)));
                either::Left(iter)
            }
            Err(error) => either::Right(std::iter::once(Err(error))),
        })
        .filter_map(|result| match result {
            Ok((txid, i, txout)) => {
                if txout.script_pubkey == *script_pubkey {
                    txos += 1;
                    total_amount += txout.value;
                    let out_point = OutPoint {
                        txid,
                        vout: i.try_into().unwrap(),
                    };
                    Some(Ok((out_point, txout.value)))
                } else {
                    None
                }
            }
            Err(error) => Some(Err(error)),
        });
    db.store_txos(txos_iter, current_height)?;
    println!(
        "Scanned {} blocks and {} transactions, found {} txos totalling {} sats.",
        block_count, tx_count, txos, total_amount
    );
    Ok(())
}

/// Sends a transaction.
///
/// Things to remember:
/// - You need to get some coins to send first, either:
///   - By mining to an address controlled by a wallet in bitcoind then send using bitcoin-cli to an address you create with `address` above.
///   - By mining directly to an address you create with `address` above (make sure you mine another 100 blocks so the coins are spendable).
fn send(_args: std::env::Args) -> Result<()> {
    todo!("Implement send once you have scan working")
}

/// Prints the balance out of database, you must call `scan` first to populate the database.
fn balance() -> Result<()> {
    let mut db = db::Db::open()?;
    let mut total = Amount::ZERO;

    for result in db.iter_unspent()?.iter()? {
        let (_prev_out, amt) = result?;
        total += amt;
    }

    println!("Balance: {}", total);
    Ok(())
}

/// Prints help menu.
fn help() -> Result<()> {
    println!("");
    println!("Usage: pico-bitcoin-wallet COMMAND");
    println!("");
    println!("Commands:");
    println!("");
    println!(" address\t: Get the wallet address.");
    println!(" balance\t: Get the current balance.");
    println!(" scan\t\t: Scan all blocks looking for relevant transactions.");
    println!(" send\t\t: Send a given amount to the address provided.");
    println!(" help\t\t: Print this help menu.");
    println!("");

    let data_dir = db::data_dir()?;
    let config_file = config::config_file()?;

    println!("Some paths you might need:");
    println!("");
    println!("data directory: {}", data_dir.display());
    println!("configuration file: {}", config_file.display());
    println!("");

    Ok(())
}

///
/// Helper functions.
///

/// Loads a private key from file.
///
/// Creates a new private key if file is not found.
#[allow(dead_code)]
fn load_private_key() -> Result<PrivateKey> {
    let sk_path = db::private_key_file()?;

    match std::fs::read_to_string(&sk_path) {
        Ok(key) => key.parse().context("failed to parse private key"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let key = PrivateKey::new(
                secp256k1::SecretKey::new(&mut rand::thread_rng()),
                Network::Regtest,
            );
            std::fs::write(&sk_path, key.to_wif().as_bytes())
                .context("failed to save private key")?;
            Ok(key)
        }
        Err(error) => Err(anyhow!(error).context("failed to read private key")),
    }
}

/// Gets an RPC client for `bitcoind`.
#[allow(dead_code)]
fn bitcoind_rpc_client() -> Result<Client> {
    let conf = config::load()?;
    let client = bitcoincore_rpc::Client::new(&conf.bitcoind_uri, conf.bitcoind_auth)
        .context("failed to connect to bitcoind")?;

    Ok(client)
}
