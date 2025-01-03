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

IDENTITY = $(aws sts get-caller-identity --query 'Account' --output text) # 951311911900

login:
	@aws ecr get-login-password | docker login -u AWS --password-stdin "https://$(IDENTITY).dkr.ecr.us-east-1.amazonaws.com"

build-push:
	docker build -t siavava-blog-api .
	docker tag siavava-blog-api:latest 951311911900.dkr.ecr.us-east-1.amazonaws.com/siavava-blog-api:latest
	docker push 951311911900.dkr.ecr.us-east-1.amazonaws.com/siavava-blog-api:latest

push:
	docker push 951311911900.dkr.ecr.us-east-1.amazonaws.com/siavava-blog-api:latest

.PHONY: build test docs style-check lint login


