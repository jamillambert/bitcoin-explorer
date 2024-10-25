//! PostgreSQL database management.
//!
//! This module provides functions for database creation.

use postgres::{Client, Error, NoTls};
use std::io::{self, Write};
use bitcoincore_rpc::{Auth, Client as BitcoinClient, RpcApi};

/// Creates a new PostgreSQL database.
///
/// # Errors
///
/// - If the connection to the PostgreSQL database fails. This can happen due to incorrect
/// connection parameters, network issues, or the PostgreSQL server being down.
///
/// - If the database cannot be created, for example if the database `bitcoin_explorer` already
/// exists, or the user postgres does not have the necessary permissions to create a new
/// database.
pub fn create_database() -> Result<(), Error> {
    print!("Enter PostgreSQL username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter PostgreSQL password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let mut client = Client::connect(
        &format!("host=localhost user={} password={}", username, password),
        NoTls,
    )?;
    client.batch_execute(
        "
        CREATE DATABASE bitcoin_explorer;
    ",
    )?;
    Ok(())
}

/// Creates new tables in the PostgreSQL database.
///
/// The database must already exist before calling this function.
///
/// # Errors
///
/// - If the connection to the PostgreSQL database fails. This can happen due to incorrect
/// connection parameters, network issues, or the PostgreSQL server being down.
///
/// - If the tables cannot be created, for example if the tables `blocks` or `transactions` already
/// exist, or the user postgres does not have the necessary permissions to create new tables.
pub fn create_tables() -> Result<(), Error> {
    let mut client = Client::connect(
        "host=localhost dbname=bitcoin_explorer user=postgres password=postgres",
        NoTls,
    )?;

    client.batch_execute(
        "
        CREATE TABLE blocks (
            id SERIAL PRIMARY KEY,
            hash VARCHAR(64) NOT NULL,
            height INTEGER NOT NULL,
            time TIMESTAMP NOT NULL
        );

        CREATE TABLE transactions (
            id SERIAL PRIMARY KEY,
            txid VARCHAR(64) NOT NULL,
            block_id INTEGER REFERENCES blocks(id),
            amount BIGINT NOT NULL,
            time TIMESTAMP NOT NULL
        );
    ",
    )?;

    Ok(())
}

/// Fetches the latest block hash from the Bitcoin Core node.
///
/// # Errors
///
/// - If the connection to the Bitcoin Core node fails. This can happen due to incorrect
/// connection parameters, network issues, or the Bitcoin Core node being down.
///
/// - If the RPC call to get the latest block hash fails.
pub fn get_latest_block_hash() -> Result<String, bitcoincore_rpc::Error> {
    let rpc_url = "http://localhost:8332";
    let rpc_auth = Auth::UserPass("rpcuser".to_string(), "rpcpassword".to_string());
    let client = BitcoinClient::new(rpc_url, rpc_auth)?;

    let block_hash = client.get_best_block_hash()?;
    Ok(block_hash.to_string())
}

/// Fetches the block height for a given block hash from the Bitcoin Core node.
///
/// # Errors
///
/// - If the connection to the Bitcoin Core node fails. This can happen due to incorrect
/// connection parameters, network issues, or the Bitcoin Core node being down.
///
/// - If the RPC call to get the block height fails.
///
/// # Panics
///
/// - If the block hash is not found in the blockchain.
/// - If the block height is not found in the blockchain.
/// - If the block height is not a valid integer.
/// - If the block height is negative.
/// - If the block height is zero.
/// - If the block height is greater than the maximum allowed block height.
/// - If the block height is less than the minimum allowed block height.
/// - If the block height is not a valid block height.
/// - If the block height is not a valid block index.
///
/// # Examples
/// ```
/// let block_hash = "0000000000000000000f1c1e0b
///
/// let block_hash = "0000000000000000000f1c1e0b...";
/// let block_height = get_block_height(block_hash).unwrap();
/// println!("Block height: {}", block_height);
/// ```
pub fn get_block_height(block_hash: &str) -> Result<u64, bitcoincore_rpc::Error> {
    let rpc_url = "http://localhost:8332";
    let rpc_auth = Auth::UserPass("rpcuser".to_string(), "rpcpassword".to_string());
    let client = BitcoinClient::new(rpc_url, rpc_auth)?;

    let block = client.get_block_info(&block_hash.parse()?)?;
    Ok(block.height as u64)
}

/// Inserts a block into the PostgreSQL database.
///
/// # Errors
///
/// - If the connection to the PostgreSQL database fails. This can happen due to incorrect
/// connection parameters, network issues, or the PostgreSQL server being down.
///
/// - If the insertion of the block fails, for example if the block already exists, or the user
/// postgres does not have the necessary permissions to insert a new block.
pub fn insert_block(hash: &str, height: i32, time: &str) -> Result<(), Error> {
    let mut client = Client::connect(
        "host=localhost dbname=bitcoin_explorer user=postgres password=postgres",
        NoTls,
    )?;

    client.execute(
        "INSERT INTO blocks (hash, height, time) VALUES ($1, $2, $3)",
        &[&hash, &height, &time],
    )?;
    Ok(())
}

/// Inserts a transaction into the PostgreSQL database.
///
/// # Errors
///
/// - If the connection to the PostgreSQL database fails. This can happen due to incorrect
/// connection parameters, network issues, or the PostgreSQL server being down.
///
/// - If the insertion of the transaction fails, for example if the transaction already exists, or
/// the user postgres does not have the necessary permissions to insert a new transaction.
pub fn insert_transaction(txid: &str, block_id: i32, amount: i64, time: &str) -> Result<(), Error> {
    let mut client = Client::connect(
        "host=localhost dbname=bitcoin_explorer user=postgres password=postgres",
        NoTls,
    )?;

    client.execute(
        "INSERT INTO transactions (txid, block_id, amount, time) VALUES ($1, $2, $3, $4)",
        &[&txid, &block_id, &amount, &time],
    )?;
    Ok(())
}

/// Fetches the block ID for a given block hash from the PostgreSQL database.
///
/// # Errors
///
/// - If the connection to the PostgreSQL database fails. This can happen due to incorrect
/// connection parameters, network issues, or the PostgreSQL server being down.
///
/// - If the query to fetch the block ID fails, for example if the block hash does not exist.
pub fn get_block_id(hash: &str) -> Result<i32, Error> {
    let mut client = Client::connect(
        "host=localhost dbname=bitcoin_explorer user=postgres password=postgres",
        NoTls,
    )?;

    let row = client.query_one("SELECT id FROM blocks WHERE hash = $1", &[&hash])?;
    Ok(row.get(0))
}