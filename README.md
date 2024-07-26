# async-duckdb

**NOTE:** THIS IS A SED/AWK-ed VERSION OF RYAN FOWLER'S `async-sqlite` CRATE (https://github.com/ryanfowler/async-sqlite); the following readme is a crude adaptation of the original `async-sqlite` readme.

[![Crates.io](https://img.shields.io/crates/v/async-duckdb)](https://crates.io/crates/async-duckdb)
[![Docs.rs](https://docs.rs/async-duckdb/badge.svg)](https://docs.rs/async-duckdb)
[![License](https://img.shields.io/crates/l/async-duckdb)](

___


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
use async_duckdb::{ClientBuilder};

let client = ClientBuilder::new()
                .path("/path/to/db.duckdb")
                .open()
                .await?;

let value: String = client.conn(|conn| {
    conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))
}).await?;

println!("Value is: {value}");
```

**POOLS CAN ONLY BE USED WITH `access_mode='read_only'` https://duckdb.org/docs/connect/concurrency.html#handling-concurrency **
A `Pool` represents a collection of background duckdb3 connections that can be
called concurrently from any thread in your program.

To create a duckdb pool and run a query:

```rust
use async_duckdb::{PoolBuilder};

let pool = PoolBuilder::new()
              .path("/path/to/db.duckdb")
              .open()
              .await?;

let value: String = pool.conn(|conn| {
    conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))
}).await?;

println!("Value is: {value}");
```

## Cargo Features

This library tries to export almost all features that the underlying
[duckdb](https://docs.rs/duckdb/latest/duckdb/) library contains.

A notable difference is that the `bundled` feature is **enabled** by default,
but can be disabled with the following line in your Cargo.toml:

```toml
async-duckdb = { version = "*", default-features = false }
```
