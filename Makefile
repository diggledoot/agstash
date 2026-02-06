# Makefile for agstash Rust project

.PHONY: build test test-coverage clean clean-coverage fmt lint check all

# Build the project
build:
	cargo build --release
	mkdir -p bin
	cp target/release/agstash bin/agstash

# Run tests
test:
	cargo test

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
	cargo tarpaulin --out html

# Clean build artifacts
clean:
	cargo clean
	rm -rf bin/

# Clean coverage files
clean-coverage:
	rm -f tarpaulin-report.html

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy

# Run all checks
check: fmt lint test

# Build all
all: build
