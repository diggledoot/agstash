# Makefile for agstash Go project

.PHONY: build install test test-coverage clean fmt check all

# Build the project
build:
	go build -o bin/agstash cmd/agstash/main.go

# Install the project
install:
	go install cmd/agstash/main.go

# Run tests
test:
	go test ./internal/... ./tests/...

# Run tests with coverage
test-coverage:
	go test -coverprofile=coverage.out ./internal/... ./tests/...
	go tool cover -html=coverage.out -o coverage.html

# Clean build artifacts
clean:
	rm -rf bin/

# Format code
fmt:
	go fmt ./...

# Run all checks
check: fmt test

# Build and install
all: build install