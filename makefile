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
format: ## Format files with rustfmt
	cargo fmt --all

.PHONY: lint
lint: ## Lint files with clippy
	cargo clippy --all-targets --all-features -- -Aclippy::style

.PHONY: test
test: ## Run unit tests
	cargo test --workspace
