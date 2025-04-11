.PHONY: all build test fmt lint check integration

all: build

build:
	cargo build

test:
	cargo test

integration:
	cargo test --test integration

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check:
	cargo check
