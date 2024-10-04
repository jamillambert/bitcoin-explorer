use postgres::{Client, NoTls, Error};

pub fn create_database() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres password=postgres", NoTls)?;

    client.batch_execute("
        CREATE DATABASE bitcoin_explorer;
    ")?;

    Ok(())
}

pub fn create_tables() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost dbname=bitcoin_explorer user=postgres password=postgres", NoTls)?;

    client.batch_execute("
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
    ")?;

    Ok(())
}
