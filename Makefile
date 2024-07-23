IMAGE_NAME := lambda

.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: container
container:
	docker build -t ${IMAGE_NAME} .

.PHONY: clean
clean:
	cargo clean
	docker rmi -f ${IMAGE_NAME}:latest