clean:
	@for x in `fd -t d -d 2`; do \
		if [ -d $$x/target ]; then \
		    echo $$x; \
			cd $$x; \
			cargo clean; \
			cd ..; \
		fi \
	done
