#!/usr/bin/env bash

for x in `fd Cargo.toml`; do
	dir=$(dirname $x)
	if [ -d $dir/target ]; then
		echo $dir
		(cd $dir && cargo clean)
	fi
done

exit 0
for x in `fd -t d -d 3`; do 
	if [ -d $$x/target ]; then 
		echo $$x; 
		(cd $$x && cargo clean); 
	fi 
done
