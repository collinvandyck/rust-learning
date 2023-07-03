clean:
	@for x in `fd -t d -d 3`; do \
		if [ -d $$x/target ]; then \
		    echo $$x; \
			(cd $$x && cargo clean); \
		fi \
	done
