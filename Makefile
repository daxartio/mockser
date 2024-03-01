DEFAULT_GOAL := all

DOCKER_IMAGE_NAME := daxart/mockser
DOCKER_BUILD_ARGS := --build-arg MOCKSER_VERSION=$(v)

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
	docker build -t $(DOCKER_IMAGE_NAME) -f Dockerfile $(DOCKER_BUILD_ARGS) .
	docker tag $(DOCKER_IMAGE_NAME) $(DOCKER_IMAGE_NAME):$(shell echo $(v) | cut -d. -f1,2)

.PHONY: docker-push
docker-push:
	docker push $(DOCKER_IMAGE_NAME):$(shell echo $(v) | cut -d. -f1,2)
	docker push $(DOCKER_IMAGE_NAME)
