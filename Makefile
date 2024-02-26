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

.PHONY: docker-build
docker-build:
	docker build -t daxart/mockser -f Dockerfile --build-arg MOCKSER_VERSION=$(v) .
	docker tag daxart/mockser daxart/mockser:$(shell echo $(v) | cut -d. -f1,2)

.PHONY: docker-push
docker-push:
	docker push daxart/mockser:$(shell echo $(v) | cut -d. -f1,2)
	docker push daxart/mockser
