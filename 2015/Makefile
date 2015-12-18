all:
	for d in day*; do (cd $$d; cargo run --release); done
build:
	for d in day*; do (cd $$d; cargo build --release); done
clean:
	for d in *; do rm -rf $$d/target; done
