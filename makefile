.DEFAULT_GOAL: help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: init-lib
init-lib: ## Create a new library crate
	@if [ -n "lib/$(NAME)" ]; then \
		cargo init --lib --vcs none lib/$(NAME); \
	else \
		echo "Usage: make init-lib NAME=[name]"; \
	fi

.PHONY: init-app
init-app: ## Create a new application crate
	@if [ -n "applications/$(NAME)" ]; then \
		cargo init --vcs none applications/$(NAME); \
	else \
		echo "Usage: make init-app NAME=[name]"; \
	fi

.PHONY: install
install: ## Install dependencies
	@echo "TODO" && exit 1

.PHONY: dev
dev: ## Run the application in development mode
	@echo "TODO" && exit 1

.PHONY: build
build: ## Build the optimized, product-ready application
	@echo "TODO" && exit 1

.PHONY: format
format: ## Format files with rustfmt
	cargo fmt --all

.PHONY: lint
lint: ## Lint files with clippy
	cargo clippy --all-targets --all-features -- -Aclippy::style

.PHONY: test
test: ## Run unit tests
	cargo test --workspace
