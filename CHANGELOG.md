# CHANGELOG

## `v0.3.0`

- upgrade duckdb to `1.4.0`

## `v0.2.0`

- upgrade duckdb to `1.3.2`
- update rust-edition to 2024 as it is 2025
- format
- update dev deps

## `v0.1.0`

- duckdb `v1.2.1` - `v1.2.0` was/is broken
- linting setup w/ clippy!
- Added `conn_for_each` and `conn_for_each_blocking` to pool - and tests
- Made pool auto add read-only config to connections if path is provided in builder otherwise memory...
