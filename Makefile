.DEFAULT_GOAL := help
CODE = avrora tests benches
POETRY_RUN = poetry run
TEST = $(POETRY_RUN) pytest tests $(args)

.PHONY: help
help: ## Show help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: all
all: format lint test  ## Run format lint test

.PHONY: install-poetry
install-poetry:  ## Install poetry
	pip install poetry

.PHONY: install
install:  ## Install dependencies
	poetry install

.PHONY: install-docs
install-docs:  ## Install docs dependencies
	poetry install --only docs

.PHONY: publish
publish:  ## Publish package
	@$(POETRY_RUN) maturin upload

.PHONY: build-rs
build-rs:  ## Build .so files
	$(POETRY_RUN) maturin develop -r

.PHONY: format-rs
format-rs:  ## Formatting rs code
	cargo fmt

.PHONY: lint-rs
lint-rs:  ## Check rs code
	cargo check
.PHONY: test
test:  ## Test with coverage
	$(TEST) --cov=./

.PHONY: test-fast
test-fast:  ## Test until error
	$(TEST) --exitfirst

.PHONY: test-failed
test-failed:  ## Test failed
	$(TEST) --last-failed

.PHONY: test-report
test-report:  ## Report testing
	$(TEST) --cov --cov-report html
	$(POETRY_RUN) python -m webbrowser 'htmlcov/index.html'

.PHONY: lint
lint:  ## Check code
	$(POETRY_RUN) ruff $(CODE)
	$(POETRY_RUN) pytest --dead-fixtures --dup-fixtures
	$(POETRY_RUN) mypy $(CODE)

.PHONY: format
format:  ## Formatting code
	$(POETRY_RUN) ruff --fix-only $(CODE)

.PHONY: bump
bump:  ## Bump version (commit and tag)
	$(POETRY_RUN) cz bump

.PHONY: clean
clean:  ## Clean
	rm -rf site || true
	rm -rf dist || true
	rm -rf htmlcov || true

.PHONY: bench
benchmark:  ## Benchmark
	$(TEST) benches
