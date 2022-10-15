.DEFAULT_GOAL := help
ROOT_DIR := $(dir $(realpath $(firstword $(MAKEFILE_LIST))))
SHELL := /usr/bin/env bash

IMAGE ?= ciservice
VERSION ?= 0.0.0-development-SNAPSHOT

PORT ?= 8000

RUST_VERSION := 1.64.0

##@ Main
.PHONY: build
build:  ## Build ciservice docker image
	@docker build \
		--build-arg RUST_VERSION=$(RUST_VERSION) \
		--build-arg ROCKET_PORT=$(PORT) \
		-f docker/Dockerfile \
		-t $(IMAGE):$(VERSION) \
		.

.PHONY: cli
cli: build  ## Start a shell inside a new ciservice container and mounts the cwd in the ciuser home dir
	@docker run --rm -it \
		-v "$(ROOT_DIR):/home/ciuser" \
		$(IMAGE):$(VERSION) \
		bash

##@ Development
DOCKER_COMPOSE := env RUST_VERSION=$(RUST_VERSION) ROCKET_PORT=$(PORT) docker compose -f $(ROOT_DIR)/docker/docker-compose.yaml

.PHONY: start
start:  ## Launch dev cluster
	@$(DOCKER_COMPOSE) up --build -d

.PHONY: stop
stop:  ## Stop dev cluster
	@$(DOCKER_COMPOSE) stop

.PHONY: restart
restart:  ## Stop and start dev cluster
	@$(DOCKER_COMPOSE) restart

.PHONY: cluster-cli
cluster-cli:  ## Start a shell inside the ciservice container of the running cluster
	@docker exec -it ciservice bash

.PHONY: down
down:  ## Destroy dev cluster (also removes volumes)
	@$(DOCKER_COMPOSE) down --volumes

.PHONY: logs
logs:  ## Tail logs of dev cluster
	@$(DOCKER_COMPOSE) logs -f

##@ Helpers
.PHONY: gen-secret
gen-secret:  ## Generate a new secret key
	@openssl rand -base64 32

.PHONY: print-%
print-%:  ## Print a Makefile var
	@echo $($*)

.PHONY: help
help:  ## Display this help
	@awk 'BEGIN { \
		FS = ":.*##"; printf "\nUSAGE: make \033[36m<target>\033[0m\n\nThis Makefile provides commands and common helpers useful to work with the ciservice stack\n" \
	} \
	/^[$$()% a-zA-Z_-]+:.*?##/ { \
		printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 \
	} \
	/^##@/ { \
		printf "\n\033[1m%s\033[0m\n", substr($$0, 5) \
	} \
	' $(MAKEFILE_LIST)
