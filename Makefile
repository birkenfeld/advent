SHELL = /bin/bash

.PHONY: all build 20*

all:
	@for d in 20*; do $(MAKE) --no-print-directory $$d || exit 1; done

build:
	@cargo build --release --workspace

20*:
	@cargo build --release -p advent-$@
	@for source in $@/src/bin/$@_day*; do \
	     day=`basename $$source .rs`; \
	     echo -e '\n\x1b[01m'$$day'\x1b[0m'; \
	     target/release/$$day $@/input/$$day || exit 1; \
	 done

time-%:
	@cargo build --release -p advent-$*
	@for source in $*/src/bin/$*_day*; do \
	     day=`basename $$source .rs`; \
	     echo -e '\n\x1b[01m'$$t'\x1b[0m'; \
	     perf stat --null target/release/$$t $*/input/$$t 2>&1 | grep elapsed; \
	 done
