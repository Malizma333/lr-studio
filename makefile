.DEFAULT_GOAL: help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: init-lib
init-lib: ## Create a new library crate
	@if [ -n "$(NAME)" ]; then \
		cargo init --lib --vcs none $(NAME); \
	else \
		echo "Usage: make init-lib NAME=[name]"; \
	fi

.PHONY: init-app
init-app: ## Create a new application crate
	@if [ -n "$(NAME)" ]; then \
		cargo init --vcs none $(NAME); \
	else \
		echo "Usage: make init-app NAME=[name]"; \
	fi

.PHONY: format
format: ## Format files with rustfmt
	cargo fmt --all

.PHONY: lint
lint: ## Lint files with clippy
	cargo clippy --all-targets --all-features -- -Aclippy::style

.PHONY: test
test: ## Run unit tests
	cargo test --workspace
