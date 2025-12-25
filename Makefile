# Makefile for building server
#
# Author: Amittai (@siavava)

.PHONY: build test docs style-check lint login

build:
	@cargo build --release

dev:
	cargo watch -x run

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc # --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings
