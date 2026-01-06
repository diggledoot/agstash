# Makefile for agstash Go project

.PHONY: build test test-coverage clean fmt check all

# Build the project
build:
	go build -o bin/agstash cmd/agstash/main.go

# Run tests
test:
	`go env GOPATH`/bin/gotestsum -- ./internal/... ./tests/...

# Run tests with coverage
test-coverage:
	`go env GOPATH`/bin/gotestsum --format=testname -- -coverprofile=coverage.out ./internal/... ./tests/...
	go tool cover -html=coverage.out -o coverage.html

# Clean build artifacts
clean:
	rm -rf bin/

# Format code
fmt:
	go fmt ./...

# Run all checks
check: fmt test

# Build all
all: build
