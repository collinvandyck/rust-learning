#!/usr/bin/env bash

for x in `fd Cargo.toml`; do
	dir=$(dirname $x)
	if [ -d $dir/target ]; then
		echo $dir
		(cd $dir && cargo clean)
	fi
done
