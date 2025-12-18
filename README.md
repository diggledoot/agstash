# agstash

`agstash` is a CLI tool designed to manage `AGENTS.md` files. It allows you to create, clean, stash, and apply these files, acting effectively as a "stash" for your agent instructions or context that you want to persist or transfer between sessions/states of a project.

## Purpose

`agstash` helps you manage AI agent instructions stored in `AGENTS.md` files:

- **Init**: Create a new `AGENTS.md` with default best practices
- **Stash**: Save your current `AGENTS.md` to global storage
- **Apply**: Restore the `AGENTS.md` from the global stash back to your project
- **Clean**: Remove the local `AGENTS.md`

## Installation

```bash
cargo install --path .
```

## Usage

```bash
agstash <COMMAND>
```

### Commands

- **`init`** - Create a new `AGENTS.md` file in the current directory
- **`clean`** - Remove the `AGENTS.md` file from the current directory
- **`stash`** - Save the `AGENTS.md` file to global storage (`~/.agstash/stashes/`)
- **`apply`** - Restore the stashed `AGENTS.md` file back to your project (with `--force` option to overwrite without prompting)
- **`uninstall`** - Remove the global `.agstash` directory and all stashed files

## Build

To build the project locally:

```bash
cargo build --release
```
