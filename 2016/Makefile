SHELL = /bin/bash
MODE = release
ifeq ($(MODE),release)
	MODEARG = --release
endif

all:
	@for d in day*; do echo -e '\n\x1b[01m'$$d; (cd $$d; cargo run $(MODEARG)); done
build:
	@for d in day*; do echo -e '\n\x1b[01m'$$d; (cd $$d; cargo build $(MODEARG)); done
time:
	@for d in day*; do echo -e '\n\x1b[01m'$$d; (cd $$d; cargo build $(MODEARG) && perf stat --null target/$(MODE)/$$d); done
clean:
	for d in *; do rm -rf $$d/target; done
