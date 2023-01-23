SHELL = /bin/bash
# can also be lto
PROFILE = release

.PHONY: all build 20*

all:
	@for year in 20*; do $(MAKE) --no-print-directory $$year || exit 1; done

build:
	@for year in 20*; do cargo build --profile $(PROFILE) -p advent-$$year || exit 1; done

20*:
	@cargo build --profile $(PROFILE) -p advent-$@
	@for source in $@/src/bin/$@_day*; do \
	     day=`basename $$source .rs`; \
	     echo -e '\n\x1b[01m'$$day'\x1b[0m'; \
	     target/$(PROFILE)/$$day || exit 1; \
	 done

time-%:
	@cargo build --profile $(PROFILE) -p advent-$*
	@for source in $*/src/bin/$*_day*; do \
	     day=`basename $$source .rs`; \
	     echo -e '\n\x1b[01m'$$t'\x1b[0m'; \
	     perf stat --null target/$(PROFILE)/$$t 2>&1 | grep elapsed; \
	 done
