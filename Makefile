SHELL := bash
CC := $(shell which cargo)
PWD := $(shell pwd)
PROJECT_NAME := $(shell pwd | sed "s#.*/##")
DOCKER_IMAGE_NAME := $(shell pwd | sed "s#.*/##" | tr [:upper:] [:lower:])
BIN := ${PROJECT_NAME}
SRC_DIR := src
LIB_DIR := 
CARGO_TOML := Cargo.toml

all: build run

# Rust code
clean:
	$(CC) clean

doc:
	$(CC) doc

fmt:
	$(CC) fmt

update:
	$(CC) update

build: fmt update clean
	$(CC) build --release

run:
	./bin/${BIN}

test: fmt
	$(CC) test --release

test-offline: fmt
	$(CC) test --release --offline

build-linux-image:
	tar cvf docker/build.tar ${SRC_DIR} ${CARGO_TOML} ${LIB_DIR}
	docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux
	rm docker/build.tar

rebuild-linux-image:
	tar cvf docker/build.tar ${SRC_DIR} ${CARGO_TOML} ${LIB_DIR}
	docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux --no-cache
	rm docker/build.tar

docker-init:
	docker run -itd -v $(shell pwd):/app --name ${DOCKER_IMAGE_NAME}-linux ${DOCKER_IMAGE_NAME}/linux /bin/bash

docker-build:
	docker start ${DOCKER_IMAGE_NAME}-linux
	docker exec ${DOCKER_IMAGE_NAME}-linux /bin/bash -c "make build"

docker-doc:
	docker start ${DOCKER_IMAGE_NAME}-linux
	docker exec ${DOCKER_IMAGE_NAME}-linux /bin/bash -c "make doc"

docker-test: fmt clean
	docker start ${DOCKER_IMAGE_NAME}-linux
	docker exec ${DOCKER_IMAGE_NAME}-linux /bin/bash -c "make test"

docker-test-offline: fmt clean
	docker start ${DOCKER_IMAGE_NAME}-linux
	docker exec ${DOCKER_IMAGE_NAME}-linux /bin/bash -c "make test-offline"
