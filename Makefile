clean:
	@for x in `fd -t d -d 1`; do \
		if [ -d $$x/target ]; then \
		    echo $$x; \
			cd $$x; \
			cargo clean; \
			cd ..; \
		fi \
	done
