default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	
clean:
	rm -rf target
	rm -rf node_modules
	
fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: default all test build clean fmt clippy 