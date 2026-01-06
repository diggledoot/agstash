# agstash

`agstash` is a CLI tool designed to manage `AGENTS.md` files. It allows you to create, clean, stash, and apply these files, acting effectively as a "stash" for your agent instructions or context that you want to persist or transfer between sessions/states of a project.

## Purpose

`agstash` helps you manage AI agent instructions stored in `AGENTS.md` files.

## Build from source:

```bash
git clone <repository-url>
cd agstash
make build
./bin/agstash --help
```

Or build directly:

```bash
go build -o agstash cmd/agstash/main.go
./agstash --help
```

## Usage

```bash
agstash <COMMAND>
```

To see available commands, run:
```bash
agstash help
```

## Build

To build the project locally:

```bash
go build -o agstash cmd/agstash/main.go
```

## Development

The project includes a Makefile with common development tasks:

```bash
make build          # Build the project
make test           # Run tests
make clean          # Clean build artifacts
```

## Testing

To run unit tests with gotestsum:

```bash
gotestsum -- ./internal/... ./tests/...
```

Or using make:

```bash
make test
```

To run all tests:

```bash
gotestsum -- ./...
```

To run tests with coverage:

```bash
make test-coverage
```