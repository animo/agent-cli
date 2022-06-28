default: format lint build e2e-test

format:
	cargo fmt

lint:
	cargo clippy

build:
	cargo build --release

test:
	cargo test

setup-test:
	docker-compose -f ./docker/docker-compose.acapy.yml up

install:
	cargo install --path .
