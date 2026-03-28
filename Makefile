# Makefile for building server
#
# Author: Amittai (@siavava)

# If the first argument is "update"...
ifeq (update,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "update"
  UPDATE_EXTRA := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  UPDATE_ARGS := $(if $(UPDATE_EXTRA),-p $(UPDATE_EXTRA))
  # ...and turn them into do-nothing targets
  $(eval $(UPDATE_ARGS):;@:)
endif

.PHONY: build dev test docs style-check lint login

build:
	@cargo build --release

dev:
	cargo watch -x run

clean:
	@cargo clean

fmt:
	@cargo +nightly fmt

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

audit:
	@cargo audit

update:
	@cargo update $(UPDATE_ARGS)
