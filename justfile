#!/usr/bin/env just --justfile
# 'justfile'
# just-repo: https://github.com/casey/just
# just-docs: https://just.systems/man/en/

@_default:
    just --list --unsorted

all:
    echo "unimplemented"

build:
    echo "unimplemented"

# ci -- often default or 'all' target
ci:
    echo "unimplemented"

lint:
    echo "unimplemented"

test:
    echo "unimplemented"

clean:
    echo "unimplemented"


NUKE:
    npx rimraf ./**/node_modules

# FORMATTING

fmt-prettier:
    npx prettier@latest --write .

ruffmt:
    ruff format .

