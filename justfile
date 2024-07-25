#!/usr/bin/env just --justfile
# 'justfile'
# just-repo: https://github.com/casey/just
# just-docs: https://just.systems/man/en/

@_default:
    just --list --unsorted

dev: fmt test

# Format
fmt:
    cargo fmt

# Format check
fmtc:
    cargo fmt -- --check

test:
    cargo test

build:
    cargo build

lint:
    cargo clippy --all-targets --all-features -- -D warnings

lintfix:
    cargo clippy --all-targets --all-features --fix -- -D warnings

ci: fmtc test lint
