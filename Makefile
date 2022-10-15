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
		--build-arg ROCKET_PORT=$(PORT) \
		--build-arg RUST_VERSION=$(RUST_VERSION) \
		-f docker/Dockerfile \
		-t $(IMAGE):$(VERSION) \
		.

.PHONY: run
run: build  ## Run ciservice
	@docker run --rm -it \
		--env ROCKET_PORT=$(PORT) \
		--env ROCKET_LOG_LEVEL=debug \
		-p "$(PORT):$(PORT)" \
		-v "$(ROOT_DIR)/static:/home/ciuser/static" \
		$(IMAGE):$(VERSION)

.PHONY: cli
cli: build  ## Start a shell inside a ciservice container
	@docker run --rm -it $(IMAGE):$(VERSION) bash

##@ Helpers
.PHONY: gen-secret
gen-secret:  ## Generate a new secret key
	@openssl rand -base64 32

.PHONY: print-%
print-%:  ## Print a MAKEFILE var
	@echo $($*)

.PHONY: help
help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage: make \033[36m<target>\033[0m\n"} /^[$$()% a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

# ##@ Development
# DOCKER_COMPOSE := env CONFLUENT_VERSION=$(CONFLUENT_VERSION) docker-compose -f $(ROOT_DIR)/docker-compose.yaml
# .PHONY: dev-cluster-start
# dev-cluster-start:  ## Launch Kafka dev cluster
# 	@$(DOCKER_COMPOSE) up -d

# .PHONY: dev-cluster-stop
# dev-cluster-stop:  ## Stop Kafka dev cluster
# 	@$(DOCKER_COMPOSE) stop

# .PHONY: dev-cluster-down
# dev-cluster-down:  ## Destroy Kafka dev cluster
# 	@$(DOCKER_COMPOSE) down

# .PHONY: dev-cluster-logs
# dev-cluster-logs:  ## Tail logs of Kafka dev cluster
# 	@$(DOCKER_COMPOSE) logs -f kafka-rest
