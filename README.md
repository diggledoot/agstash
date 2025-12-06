# agstash

`agstash` is a CLI tool designed to manage `AGENTS.md` files. It allows you to create, clean, stash, and apply these files, acting effectively as a "stash" for your agent instructions or context that you want to persist or transfer between sessions/states of a project, separate from your main version control.

## Purpose

The primary purpose of this tool is to provide a workflow for managing `AGENTS.md` files. These files are typically used to store instructions, context, or rules for AI agents working on your codebase. `agstash` helps you:

- **Init**: Quickly create a new `AGENTS.md` with default best practices.
- **Stash**: Save your current `AGENTS.md` to a global storage (`~/.agstash`) keyed by your project folder name. This is useful if you want to temporarily save the state of your agent instructions.
- **Apply**: Restore the `AGENTS.md` from the global stash back to your project.
- **Clean**: Remove the local `AGENTS.md`.

## Installation

```bash
cargo install --path .
```

## Uninstallation

To completely remove the application and its data:

1. Run the app's uninstall command to clean up data:
   ```bash
   agstash uninstall
   ```
2. Uninstall the binary using cargo:
   ```bash
   cargo uninstall agstash
   ```

## Usage

```bash
agstash <COMMAND>
```

### Commands

- `init`
    - Initialize a new `AGENTS.md` file in the current directory (or project root).
    
- `clean`
    - Remove the `AGENTS.md` file from the current directory.

- `stash`
    - Stash the `AGENTS.md` file globally. It identifies the project root and saves the file to `~/.agstash/stashes/stash-<project_name>.md`.


- `apply`
    - Apply the stashed `AGENTS.md` file back to the project root.
    - **Note**: If `AGENTS.md` already exists, you will be prompted to overwrite it.

- `uninstall`
    - Remove the global `.agstash` directory and all stashed files.
    - **Warning**: This action is irreversible.

## Build

To build the project locally:

```bash
cargo build --release
```
