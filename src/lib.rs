//! # async-duckdb
//!
//! A library to interact with sqlite from an async context.
//!
//! This library is tested on both [tokio](https://docs.rs/tokio/latest/tokio/)
//! and [async_std](https://docs.rs/async-std/latest/async_std/), however
//! it should be compatible with all async runtimes.
//!
//! ## Usage
//!
//! A `Client` represents a single background duckdb connection that can be called
//! concurrently from any thread in your program.
//!
//! To create a sqlite client and run a query:
//!
//! ```rust
//! use async_duckdb::{ClientBuilder};
//!
//! # async fn run() -> Result<(), async_duckdb::Error> {
//! let client = ClientBuilder::new()
//!                 .path("/path/to/db.duckdb")
//!                 .open()
//!                 .await?;
//!
//! let value: String = client.conn(|conn| {
//!     conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))
//! }).await?;
//!
//! println!("Value is: {value}");
//! # Ok(())
//! }
//! ```
//!
//! ## Cargo Features
//!
//! This library tries to export almost all features that the underlying
//! [duckdb](https://docs.rs/duckdb/latest/duckdb/) library contains.
//!
//! A notable difference is that the `bundled` feature is **enabled** by default,
//! but can be disabled with the following line in your Cargo.toml:
//!
//! ```toml
//! async-duckdb = { version = "*", default-features = false }
//! ```

pub use duckdb;
pub use duckdb::{Config, Connection};

mod client;
mod error;
mod pool;

pub use client::{Client, ClientBuilder};
pub use pool::{Pool, PoolBuilder};
pub use error::Error;
