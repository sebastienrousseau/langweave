# Makefile using cargo for managing builds and dependencies in a Rust project.

# Default target executed when no arguments are given to make.
.PHONY: all
all: help ## Display this help.

# Build the project including all workspace members.
.PHONY: build
build: ## Build the project.
	@echo "Building all project components..."
	@cargo build --all

# Lint the project with stringent rules using Clippy, install Clippy if not present.
.PHONY: lint
lint: ensure-clippy ## Lint the project with Clippy.
	@echo "Linting with Clippy..."
	@cargo clippy --all-features --all-targets --all -- \
		--deny clippy::dbg_macro --deny clippy::unimplemented --deny clippy::todo --deny warnings \
		--deny missing_docs --deny broken_intra_doc_links --forbid unused_must_use --deny clippy::result_unit_err

# Run all unit and integration tests in the project.
.PHONY: test
test: ## Run tests for the project.
	@echo "Running tests..."
	@cargo test

# Check the project for errors without producing outputs.
.PHONY: check
check: ## Check the project for errors without producing outputs.
	@echo "Checking code formatting..."
	@cargo check

# Format all code in the project according to rustfmt's standards, install rustfmt if not present.
.PHONY: format
format: ensure-rustfmt ## Format the code.
	@echo "Formatting all project components..."
	@cargo fmt --all

# Check code formatting without making changes, with verbose output, install rustfmt if not present.
.PHONY: format-check-verbose
format-check-verbose: ensure-rustfmt ## Check code formatting with verbose output.
	@echo "Checking code format with verbose output..."
	@cargo fmt --all -- --check --verbose

# Apply fixes to the project using cargo fix, install cargo-fix if necessary.
.PHONY: fix
fix: ensure-cargo-fix ## Automatically fix Rust lint warnings using cargo fix.
	@echo "Applying cargo fix..."
	@cargo fix --all

# Use cargo-deny to check for security vulnerabilities, licensing issues, and more, install if not present.
.PHONY: deny
deny: ensure-cargo-deny ## Run cargo deny checks.
	@echo "Running cargo deny checks..."
	@cargo deny check

# Check for outdated dependencies only for the root package, install cargo-outdated if necessary.
.PHONY: outdated
outdated: ensure-cargo-outdated ## Check for outdated dependencies for the root package only.
	@echo "Checking for outdated dependencies..."
	@cargo outdated --root-deps-only

# Check architectural boundaries to prevent Core from importing UI/Network/Filesystem layers.
.PHONY: arch-check
arch-check: ## Check architectural layer boundaries.
	@echo "Checking architectural boundaries..."
	@python3 scripts/check_architecture_simple.py

# Check full architectural compliance (requires external dependencies).
.PHONY: arch-check-full
arch-check-full: ensure-python-deps ## Check full architectural compliance.
	@echo "Running full architectural analysis..."
	@python3 scripts/check_architecture.py

# Installation checks and setups
.PHONY: ensure-clippy ensure-rustfmt ensure-cargo-fix ensure-cargo-deny ensure-cargo-outdated ensure-python-deps
ensure-clippy:
	@cargo clippy --version || rustup component add clippy

ensure-rustfmt:
	@cargo fmt --version || rustup component add rustfmt

ensure-cargo-fix:
	@cargo fix --version || rustup component add rustfix

ensure-cargo-deny:
	@command -v cargo-deny || cargo install cargo-deny

ensure-cargo-outdated:
	@command -v cargo-outdated || cargo install cargo-outdated

ensure-python-deps:
	@python3 -c "import toml" 2>/dev/null || echo "Install toml: pip install toml"

# =============================================================================
# CI PIPELINE VALIDATION TARGETS
# =============================================================================

# Run all CI checks locally (matches GitHub Actions pipeline)
.PHONY: ci-check
ci-check: ci-lint ci-format-check ci-build ci-test-coverage ci-security-audit ## Run complete CI validation locally.
	@echo "🚀 All CI checks completed successfully!"
	@echo "✅ Repository meets langweave v0.0.2 standards compliance"

# CI Linting with zero-warning policy
.PHONY: ci-lint
ci-lint: ensure-clippy ## Run CI linting with zero-warning policy.
	@echo "📋 Running Clippy with zero-warning policy..."
	@RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features --workspace -- -D warnings
	@echo "✅ All linting checks passed"

# CI Format check (fail on unformatted code)
.PHONY: ci-format-check
ci-format-check: ensure-rustfmt ## Check formatting (fail on unformatted code).
	@echo "🎨 Enforcing code formatting standards..."
	@cargo fmt --all -- --check || (echo "❌ Code is not formatted. Run 'cargo fmt --all'" && exit 1)
	@echo "✅ All code is properly formatted"

# CI Build with strict warnings
.PHONY: ci-build
ci-build: ## Build all targets with warnings-as-errors.
	@echo "🔨 Building all targets with strict error handling..."
	@RUSTFLAGS="-D warnings" cargo build --all-targets --all-features --workspace
	@echo "✅ Build completed successfully"

# CI Test coverage with 100% requirement
.PHONY: ci-test-coverage
ci-test-coverage: ensure-cargo-llvm-cov ## Run tests with 100% coverage requirement.
	@echo "🧪 Running test suite with coverage analysis..."
	@cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
	@cargo llvm-cov --all-features --workspace --summary-only > coverage_summary.txt
	@COVERAGE=$$(grep "^TOTAL" coverage_summary.txt | grep -oE '[0-9]+\.[0-9]+%' | tail -1 | sed 's/%//'); \
	echo "Current line coverage: $${COVERAGE}%"; \
	if awk "BEGIN {exit !($${COVERAGE} < 100)}"; then \
		echo "❌ Coverage $${COVERAGE}% is below required 100% threshold"; \
		echo "🎯 CRITICAL: All code must be covered by tests"; \
		exit 1; \
	else \
		echo "✅ Coverage $${COVERAGE}% meets required 100% threshold"; \
	fi

# CI Security audit (strict enforcement)
.PHONY: ci-security-audit
ci-security-audit: ensure-cargo-audit ensure-cargo-deny ## Run security audit with strict enforcement.
	@echo "🔒 Running security audit with strict enforcement..."
	@echo "🚫 NO SECURITY BYPASSES ALLOWED"
	@cargo audit
	@cargo deny check
	@echo "✅ All security checks passed"

# Check for unused dependencies
.PHONY: ci-unused-deps
ci-unused-deps: ensure-cargo-machete ## Check for unused dependencies.
	@echo "🔍 Checking for unused dependencies..."
	@cargo machete
	@echo "✅ No unused dependencies found"

# Run examples to verify they work
.PHONY: ci-examples
ci-examples: ## Test all examples.
	@echo "📋 Testing all examples..."
	@cargo run --example basic_usage_example --all-features
	@cargo run --example error_example --all-features
	@cargo run --example language_detector_example --all-features
	@echo "✅ All examples run successfully"

# =============================================================================
# TOOL INSTALLATION HELPERS
# =============================================================================

.PHONY: ensure-cargo-llvm-cov ensure-cargo-audit ensure-cargo-machete
ensure-cargo-llvm-cov:
	@command -v cargo-llvm-cov || cargo install cargo-llvm-cov

ensure-cargo-audit:
	@command -v cargo-audit || cargo install cargo-audit --locked

ensure-cargo-machete:
	@command -v cargo-machete || cargo install cargo-machete --locked

# =============================================================================
# DEVELOPMENT HELPERS
# =============================================================================

# Fix common issues automatically
.PHONY: fix-all
fix-all: format fix ## Fix formatting and linting issues automatically.
	@echo "✅ Applied all automatic fixes"

# Quick development check (faster than full CI)
.PHONY: quick-check
quick-check: ci-format-check check lint ## Quick development validation.
	@echo "✅ Quick validation completed"

# Prepare for commit (run before git commit)
.PHONY: pre-commit
pre-commit: fix-all ci-check ## Prepare for commit (format, fix, and validate).
	@echo "🚀 Ready for commit!"

# Help target to display callable targets and their descriptions.
.PHONY: help
help: ## Display this help.
	@echo "Usage: make [target]..."
	@echo ""
	@echo "CI Pipeline Targets:"
	@echo "  ci-check                      Run complete CI validation locally"
	@echo "  ci-lint                       Run CI linting with zero-warning policy"
	@echo "  ci-format-check              Check formatting (fail on unformatted code)"
	@echo "  ci-build                     Build with warnings-as-errors"
	@echo "  ci-test-coverage             Run tests with 100% coverage requirement"
	@echo "  ci-security-audit            Run security audit with strict enforcement"
	@echo ""
	@echo "Development Targets:"
	@awk 'BEGIN {FS = ":.*?##"} /^[a-zA-Z_-]+:.*?##/ {printf "  %-30s %s\n", $$1, $$2}' $(MAKEFILE_LIST)