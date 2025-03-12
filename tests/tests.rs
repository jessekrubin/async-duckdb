#![allow(clippy::unwrap_used)]
#![allow(clippy::unnecessary_wraps)]

use std::collections::HashSet;

use async_duckdb::{ClientBuilder, Error, PoolBuilder};
#[test]
fn test_blocking_client() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let client = ClientBuilder::new()
        .path(tmp_dir.path().join("duck.db"))
        .open_blocking()
        .expect("client unable to be opened");
    client
        .conn_blocking(|conn| {
            conn.execute(
                "CREATE TABLE testing (id INTEGER PRIMARY KEY, val TEXT NOT NULL)",
                [],
            )?;
            conn.execute("INSERT INTO testing VALUES (1, ?)", ["value1"])
        })
        .expect("writing schema and seed data");

    client
        .conn_blocking(|conn| {
            let val: String =
                conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))?;
            assert_eq!(val, "value1");
            Ok(())
        })
        .expect("querying for result");

    client.close_blocking().expect("closing client conn");
}

macro_rules! async_test {
    ($name:ident) => {
        paste::item! {
            #[::core::prelude::v1::test]
            fn [< $name _async_std >] () {
                ::async_std::task::block_on($name());
            }

            #[::core::prelude::v1::test]
            fn [< $name _tokio >] () {
                ::tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on($name());
            }
        }
    };
}

async_test!(test_config_fn);
async_test!(test_concurrency);
async_test!(test_pool);
async_test!(test_pool_conn_for_each);

async fn test_config_fn() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let client = ClientBuilder::new()
        .path(tmp_dir.path().join("duck.db"))
        .flagsfn(|| duckdb::Config::default().default_order(duckdb::DefaultOrder::Desc))
        .open()
        .await
        .expect("client unable to be opened");

    let mode: String = client
        .conn(|conn| {
            conn.query_row(
                "SELECT value from duckdb_settings() where name = 'default_order';",
                [],
                |row| row.get(0),
            )
        })
        .await
        .expect("client unable to fetch journal_mode");
    assert_eq!(mode, "desc");
}

async fn test_concurrency() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let client = ClientBuilder::new()
        .path(tmp_dir.path().join("duck.db"))
        .open()
        .await
        .expect("client unable to be opened");

    client
        .conn(|conn| {
            conn.execute(
                "CREATE TABLE testing (id INTEGER PRIMARY KEY, val TEXT NOT NULL)",
                [],
            )?;
            conn.execute("INSERT INTO testing VALUES (1, ?)", ["value1"])
        })
        .await
        .expect("writing schema and seed data");

    let fs = (0..10).map(|_| {
        client.conn(|conn| {
            let val: String =
                conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))?;
            assert_eq!(val, "value1");
            Ok(())
        })
    });
    futures_util::future::join_all(fs)
        .await
        .into_iter()
        .collect::<Result<(), Error>>()
        .expect("collecting query results");
}

// =================================
// DUCK DB POOLS ARE NOT RECOMMENDED
// =================================
async fn test_pool() {
    let tmp_dir = tempfile::tempdir().unwrap();
    // create dumb data base using client and write stuff

    let client = ClientBuilder::new()
        .path(tmp_dir.path().join("duck.db"))
        .open()
        .await
        .expect("client unable to be opened");
    client
        .conn(|conn| {
            conn.execute(
                "CREATE TABLE testing (id INTEGER PRIMARY KEY, val TEXT NOT NULL)",
                [],
            )?;
            conn.execute("INSERT INTO testing VALUES (1, ?)", ["value1"])
        })
        .await
        .expect("writing schema and seed data");

    client.close().await.expect("client unable to be closed");
    let pool = async_duckdb::PoolBuilder::new()
        .path(tmp_dir.path().join("duck.db"))
        .num_conns(2)
        .open()
        .await
        .expect("client unable to be opened");

    let fs = (0..10).map(|_| {
        pool.conn(|conn| {
            let val: String =
                conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))?;
            assert_eq!(val, "value1");
            Ok(())
        })
    });
    futures_util::future::join_all(fs)
        .await
        .into_iter()
        .collect::<Result<(), Error>>()
        .expect("collecting query results");
}

#[test]
fn test_blocking_pool() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let client = ClientBuilder::new()
        .path(tmp_dir.path().join("duck.db"))
        .open_blocking()
        .expect("client unable to be opened");
    client
        .conn_blocking(|conn| {
            conn.execute(
                "CREATE TABLE testing (id INTEGER PRIMARY KEY, val TEXT NOT NULL)",
                [],
            )?;
            conn.execute("INSERT INTO testing VALUES (1, ?)", ["value1"])
        })
        .expect("writing schema and seed data");

    client.close_blocking().expect("client unable to be closed");
    let pool = PoolBuilder::new()
        .path(tmp_dir.path().join("duck.db"))
        .open_blocking()
        .expect("client unable to be opened");

    pool.conn_blocking(|conn| {
        let val: String =
            conn.query_row("SELECT val FROM testing WHERE id=?", [1], |row| row.get(0))?;
        assert_eq!(val, "value1");
        Ok(())
    })
    .expect("querying for result");
    pool.close_blocking().expect("closing client conn");
}

async fn test_pool_conn_for_each() {
    /// check that the json extension is loaded
    fn check_fn(conn: &duckdb::Connection) -> Result<Vec<String>, duckdb::Error> {
        let mut stmt = conn
            .prepare_cached("SELECT * from duckdb_extensions() where loaded=true;")
            .unwrap();
        let names = stmt
            .query_map([], |row| row.get("extension_name"))
            .unwrap()
            .map(|r| r.unwrap())
            .collect::<Vec<String>>();
        Ok(names)
    }

    // make dummy db
    let tmp_dir = tempfile::tempdir().unwrap();
    {
        let client_res = ClientBuilder::new()
            .path(tmp_dir.path().join("duckdb.db"))
            .open_blocking();
        // must do this if we are doing all the tests all at once
        if let Ok(client) = client_res {
            client
                .conn_blocking(|conn| {
                    conn.execute(
                        "CREATE TABLE testing (id INTEGER PRIMARY KEY, val TEXT NOT NULL)",
                        [],
                    )?;
                    conn.execute("INSERT INTO testing VALUES (1, ?)", ["value1"])
                })
                .expect("writing schema and seed data");
            client.close_blocking().expect("closing client conn");
        }
    }

    let pool = PoolBuilder::new()
        // .path(tmp_dir.path().join("duckdb.db"))
        .num_conns(2)
        // .flagsfn(|| Ok(Config::default()))
        .open()
        .await
        .expect("pool unable to be opened");

    let load_json_ext = move |conn: &duckdb::Connection| {
        conn.execute_batch(
            "INSTALL arrow;
            LOAD arrow;
            "
        )
    };
    pool.conn_for_each(load_json_ext).await;

    let res = pool.conn_for_each(check_fn).await;
    for r in res {
        let expected =  vec!["arrow", "core_functions"].into_iter().map(|s| s.to_string()).collect::<HashSet<String>>();
        let extensions_queried = r.unwrap().into_iter().collect::<HashSet<String>>();
        assert_eq!(extensions_queried, expected);
    }
    // cleanup
    pool.close().await.expect("closing client conn");
}
