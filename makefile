.DEFAULT_GOAL: help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: install
install: ## Install development tooling locally at .cargo
	# Test coverage
	cargo install cargo-tarpaulin --version 0.35.0 --locked --root .cargo

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
test: ## Run tests (set CRATE for a specific crate)
	@if [ -z $(CRATE) ]; then\
		cargo test --workspace;\
	else\
		cargo test -p $(CRATE) -- --no-capture;\
	fi

.PHONY: coverage
coverage: ## Run coverage on tests, with report outputted to target/coverage (set CRATE for a specific crate)
	@if [ -z $(CRATE) ]; then\
		.cargo/bin/cargo-tarpaulin --workspace -o Html --output-dir target/coverage;\
	else\
		.cargo/bin/cargo-tarpaulin -p $(CRATE) -o Html --output-dir target/coverage;\
	fi

.PHONY: benchmark
benchmark: ## Run benchmarks, with report outputted to target/benchmarks (set CRATE for a specific crate)
	@if [ -z $(CRATE) ]; then\
		cargo bench --workspace;\
	else\
		cargo bench -p $(CRATE);\
	fi
	mv target/criterion target/benchmarks
