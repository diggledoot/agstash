.PHONY: all build release run test check fmt clippy clean update help

# Default target
all: help

# Build the project
build:
	cargo build

# Build for release
release:
	cargo build --release

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Check for errors without generating code (faster than build)
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy for linting
clippy:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Update dependencies
update:
	cargo update

# Show help
help:
	@echo "Available targets:"
	@echo "  build   - Build the project project"
	@echo "  release - Build the project in release mode"
	@echo "  run     - Run the project"
	@echo "  test    - Run tests"
	@echo "  check   - Check code for errors"
	@echo "  fmt     - Format code"
	@echo "  clippy  - Run linter"
	@echo "  clean   - Clean build artifacts"
	@echo "  update  - Update dependencies"
	@echo "  help    - Show this help message"
