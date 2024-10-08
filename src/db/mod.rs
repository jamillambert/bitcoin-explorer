//! PostgreSQL database management.
//!
//! This module provides functions for database creation.

use postgres::{Client, Error, NoTls};

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
    let mut client = Client::connect("host=localhost user=postgres password=postgres", NoTls)?;
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
