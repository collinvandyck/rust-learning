build:
	cargo bundle

deps:
	#!/usr/bin/env bash
	if ! command -v cargo-binstall %>/dev/null; then
		cargo install cargo-binstall
	fi
	cargo binstall -y cargo-bundle
