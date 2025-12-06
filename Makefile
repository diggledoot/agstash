.PHONY: all build run test check fmt clippy clean help

# Default target
all: help

# Build the project
build:
	cargo build

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

# Show help
help:
	@echo "Available targets:"
	@echo "  build   - Build the project project"
	@echo "  run     - Run the project"
	@echo "  test    - Run tests"
	@echo "  check   - Check code for errors"
	@echo "  fmt     - Format code"
	@echo "  clippy  - Run linter"
	@echo "  clean   - Clean build artifacts"
	@echo "  help    - Show this help message"
