.PHONY: all debug bench test update doc clean

all:
	export RUSTFLAGS=-Awarnings
	cargo +nightly build
	cargo +nightly run 

release:
	cargo +nightly build --release
	cargo +nightly run --release

test:
	cargo +nightly test --release -- --nocapture

update:
	cargo +nightly update

clean:
	cargo +nightly clean
