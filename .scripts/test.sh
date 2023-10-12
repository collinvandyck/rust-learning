#!/usr/bin/env bash

set -e
set -u

if [ -f Cargo.toml ]; then
	exec cargo test
fi


