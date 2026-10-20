.PHONY: all build test check fmt clippy clean

all: build

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test

check:
	cargo check --all-targets

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean
