www:
	#!/usr/bin/env bash
	set -euxo pipefail
	export NODE_OPTIONS=--openssl-legacy-provider
	cd www
	npm run start

build:
	wasm-pack build
