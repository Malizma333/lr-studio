.DEFAULT_GOAL: help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: init-crate
init-crate: ## Create a new create to use throughout the project
	@if [ -n "crates/$(NAME)" ]; then \
		cargo init --lib --vcs none crates/$(NAME); \
	else \
		echo "Usage: make init-crate NAME=[name]"; \
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

.PHONY: doc
doc: ## Generate documentation and open in the browser
	cd crates && cargo doc --no-deps --open

.PHONY: format
format: ## Format files with rustfmt
	cd crates && cargo fmt --all

.PHONY: lint
lint: ## Lint files with clippy
	cd crates && cargo clippy --all-targets --all-features -- -Aclippy::style

.PHONY: test
test: ## Run unit tests
	cd crates && cargo test --workspace
