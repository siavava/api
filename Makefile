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


build-docker-local:
	# docker build -t siavava-blog-api .
build-docker:
	docker buildx build --push --platform linux/arm64/v8,linux/amd64 --tag siavava-blog-api .  
	docker tag siavava-blog-api:latest 951311911900.dkr.ecr.us-east-1.amazonaws.com/siavava-blog-api:latest
	docker push 951311911900.dkr.ecr.us-east-1.amazonaws.com/siavava-blog-api:latest

push:
	docker push 951311911900.dkr.ecr.us-east-1.amazonaws.com/siavava-blog-api:latest

.PHONY: build test docs style-check lint login


