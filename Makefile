build:
	@cargo build --release

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	cargo watch -x run

IDENTITY = $(aws sts get-caller-identity --query 'Account' --output text)

login:
	@aws ecr get-login-password | docker login -u AWS --password-stdin "https://$(IDENTITY).dkr.ecr.us-east-1.amazonaws.com"

.PHONY: build test docs style-check lint login

