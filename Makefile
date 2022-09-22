.DEFAULT_GOAL := help

FORMATTING_BEGIN_YELLOW = \033[0;33m
FORMATTING_BEGIN_BLUE = \033[36m
FORMATTING_END = \033[0m

.PHONY: help
help:
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make ${FORMATTING_BEGIN_BLUE}<target>${FORMATTING_END}\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  ${FORMATTING_BEGIN_BLUE}%-46s${FORMATTING_END} %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Initialize
.PHONY: init
init: ## Initialize environment
	rustup update nightly
	rustup default nightly

##@ Format
.PHONY: fmt
fmt: ## Format
	cargo +nightly fmt

##@ Environment
.PHONY: clean
clean: ## Delete the entire target directory.
	cargo clean

##@ Building
.PHONY: build
build: clean check  ## Build the project in development mode
	cargo build

##@ Check Style
.PHONY: check
check: ## Check Style
	cargo +nightly fmt --all -- --check

##@ Testing
.PHONY: test
test: build ## Run the unit test suite
	cargo test