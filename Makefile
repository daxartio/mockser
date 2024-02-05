DEFAULT_GOAL := all

.PHONY: all
all: fmt check test

.PHONY: check
check:
	cargo +nightly fmt --all -- --check
	cargo clippy --all-features --all-targets -- -D warnings

.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt:
	cargo +nightly fmt --all
