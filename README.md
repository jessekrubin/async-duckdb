# async-duckdb

A library to interact with duckdb from an async context.

This library is tested on both [tokio](https://docs.rs/tokio/latest/tokio/)
and [async_std](https://docs.rs/async-std/latest/async_std/), however
it should be compatible with all async runtimes.

## Install

Add `async-duckdb` to your "dependencies" in your Cargo.toml file.

This can be done by running the command:

```
cargo add async-duckdb
```

## Usage

A `Client` represents a single background duckdb connection that can be called
concurrently from any thread in your program.

To create a duckdb client and run a query:

```rust
use async_duckdb::{ClientBuilder, JournalMode};

let client = ClientBuilder::new()
                .path("/path/to/db.duckdb3")
                .journal_mode(JournalMode::Wal)
                .open()
                .await?;

let value: String = client.conn(|conn| {
    conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))
}).await?;

println!("Value is: {value}");
```

A `Pool` represents a collection of background duckdb3 connections that can be
called concurrently from any thread in your program.

To create a duckdb pool and run a query:

```rust
use async_duckdb::{JournalMode, PoolBuilder};

let pool = PoolBuilder::new()
              .path("/path/to/db.duckdb3")
              .journal_mode(JournalMode::Wal)
              .open()
              .await?;

let value: String = pool.conn(|conn| {
    conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))
}).await?;

println!("Value is: {value}");
```

## Cargo Features

This library tries to export almost all features that the underlying
[ruduckdb](https://docs.rs/ruduckdb/latest/ruduckdb/) library contains.

A notable difference is that the `bundled` feature is **enabled** by default,
but can be disabled with the following line in your Cargo.toml:

```toml
async-duckdb = { version = "*", default-features = false }
```
