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

all: build

.PHONY: build dev test docs style-check lint audit

build:
	@cargo build --release

debug-build:
	cargo build --verbose

dev:
	cargo watch -x run

clean:
	@cargo clean

fmt:
	@cargo +nightly fmt

TESTS = ""
ifeq (test,$(firstword $(MAKECMDGOALS)))
  ifneq (,$(filter verbose,$(MAKECMDGOALS)))
    $(eval verbose:;@:)
    TEST_VERBOSE := 1
  endif
endif
test:
ifeq ($(TEST_VERBOSE),1)
	@cargo test $(TESTS) --offline -- --color=always --nocapture
else
	@cargo test $(TESTS) --offline -- --color=always --nocapture 2>&1 \
		| awk '/Running /{name=$$NF; gsub(/.*\//, "", name); sub(/-[0-9a-f]+\)$$/, "", name); gsub(/_/, "::", name); next} /^test result:/ && /[1-9][0-9]* passed/{printf "  %-24s%3d passed;  %3d failed;  %3d ignored;  %3d measured;  %3d filtered out;  finished in %s\n", name, $$4+0, $$6+0, $$8+0, $$10+0, $$12+0, $$17}'
endif

test-all: lint audit test

docs: build
	@cargo doc # --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo +nightly fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- --no-deps -D warnings

audit:
	cargo audit

update:
	@cargo update $(UPDATE_ARGS)
