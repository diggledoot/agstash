# Makefile for agstash Go project

# Build the project
build:
	go build -o bin/agstash cmd/agstash/main.go

# Install the project
install:
	go install cmd/agstash/main.go

# Run tests
test:
	go test ./internal/... ./tests/...

# Run all tests including verbose output
test-verbose:
	go test -v ./internal/... ./tests/...

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

# Vet code
vet:
	go vet ./...

# Run all checks
check: fmt vet test

# Build and install
all: build install