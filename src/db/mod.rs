//! PostgreSQL database management.
//!
//! This module provides functions for database creation.

use postgres::{Client, Error, NoTls};
use std::io::{self, Write};

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
