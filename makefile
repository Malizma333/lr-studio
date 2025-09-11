.DEFAULT_GOAL: help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: new-lib
new-lib: ## Create a new library to use throughout the project
	@if [ -n "$(NAME)" ]; then \
		cargo init --lib --vcs none $(NAME); \
	else \
		echo "Usage: make new-lib NAME=[name]"; \
	fi


.PHONY: install
install: ## Install dependencies

.PHONY: dev
dev: ## Run the application in development mode

.PHONY: build
build: ## Build the optimized, product-ready application

.PHONY: format
format: ## Format all files with cargo and dprint
	-cd vector2d && cargo fmt

.PHONY: lint
lint: ## Lint rust files with clippy
	-cd vector2d && cargo clippy

.PHONY: test
test: ## Run rust unit tests
	-cd vector2d && cargo test

.PHONY: precommit
precommit: test lint format ## Run tests, then lint and format code