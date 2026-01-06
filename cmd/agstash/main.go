package main

import (
	"flag"
	"fmt"
	"os"

	"agstash/internal/commands"
)

// assert function for safety checks - crashes on failure
func assert(condition bool, message string) {
	if !condition {
		fmt.Fprintf(os.Stderr, "Assertion failed: %s\n", message)
		os.Exit(1)
	}
}

func main() {
	// Assert preconditions
	assert(len(os.Args) >= 1, "os.Args should always have at least one element (program name)")

	// Parse the flags to get the command
	if len(os.Args) < 2 {
		printUsage()
		os.Exit(1)
	}

	// Extract command and subcommand args
	command := os.Args[1]
	subArgs := os.Args[2:]

	// Assert command is not empty
	assert(command != "", "Command should not be empty at this point")

	// Handle commands
	switch command {
	case "init":
		handleInitCommand(subArgs)
	case "clean":
		handleCleanCommand(subArgs)
	case "stash":
		handleStashCommand(subArgs)
	case "apply":
		handleApplyCommand(subArgs)
	case "uninstall":
		handleUninstallCommand(subArgs)
	case "help":
		printUsage()
	default:
		fmt.Fprintf(os.Stderr, "Unknown command: %s\n", command)
		printUsage()
		os.Exit(1)
	}
}

func printUsage() {
	usage := `
Usage: agstash <command> [options]

Available Commands:
  init        Initialize a new AGENTS.md file in the current directory
  clean       Remove the AGENTS.md file from the current directory
  stash       Stash the AGENTS.md file to a global location for later retrieval
  apply       Apply a previously stashed AGENTS.md file to the current directory
  uninstall   Remove the global .agstash directory and all stashed files
  help        Show this help message
`
	fmt.Println(usage)
}

func printInitHelp() {
	help := `Usage: agstash init [flags]

Initialize a new AGENTS.md file in the current directory if one doesn't exist.

When an AGENTS.md file already exists in the current directory, the command will prompt
for confirmation before overwriting it. You will be asked to type 'yes' to confirm.

Flags:
  -f, --force    Overwrite existing AGENTS.md file without prompting for confirmation

The AGENTS.md file contains guidelines for AI agents working in the project.

Examples:
  agstash init                    # Create AGENTS.md in current directory
  agstash init --force            # Create AGENTS.md without confirmation
  agstash init -f                 # Same as above, using short flag
`
	fmt.Println(help)
}

func printCleanHelp() {
	help := `Usage: agstash clean

Remove the AGENTS.md file from the current directory if it exists.

Examples:
  agstash clean                   # Remove AGENTS.md from current directory
`
	fmt.Println(help)
}

func printStashHelp() {
	help := `Usage: agstash stash

Stash the AGENTS.md file from the current directory to a global location for later retrieval.

The file is stored in ~/.agstash/stashes/stash-<project-name>.md

Examples:
  agstash stash                   # Stash AGENTS.md for current project
`
	fmt.Println(help)
}

func printApplyHelp() {
	help := `Usage: agstash apply [flags]

Apply a previously stashed AGENTS.md file from the global location back to the current directory.

When an AGENTS.md file already exists in the current directory, the command will prompt
for confirmation before overwriting it. You will be asked to type 'yes' to confirm.

Flags:
  -f, --force    Overwrite existing AGENTS.md file without prompting for confirmation

Examples:
  agstash apply                 # Apply stashed AGENTS.md with confirmation prompt
  agstash apply --force         # Apply stashed AGENTS.md without confirmation
  agstash apply -f              # Same as above, using short flag
`
	fmt.Println(help)
}

func printUninstallHelp() {
	help := `Usage: agstash uninstall

Remove the global .agstash directory and all stashed files from your home directory.

WARNING: This will permanently delete all stashed AGENTS.md files.

Examples:
  agstash uninstall             # Remove all stashed files and .agstash directory
`
	fmt.Println(help)
}

func handleInitCommand(args []string) {
	// Assert preconditions
	assert(args != nil, "args should not be nil")

	// Parse flags for init command
	initFlags := flag.NewFlagSet("init", flag.ExitOnError)
	force := initFlags.Bool("force", false, "Overwrite existing AGENTS.md file without prompting for confirmation")
	initFlags.BoolVar(force, "f", false, "Overwrite existing AGENTS.md file without prompting for confirmation")
	helpRequested := initFlags.Bool("help", false, "Show help for init command")
	initFlags.BoolVar(helpRequested, "h", false, "Show help for init command")

	initFlags.Parse(args)

	// Check if help was requested
	if *helpRequested {
		printInitHelp()
		return
	}

	err := commands.HandleInit(*force)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	// Assert postcondition - the command should complete without error
	assert(err == nil, "HandleInit should not return an error")
}

func handleCleanCommand(args []string) {
	// Assert preconditions
	assert(args != nil, "args should not be nil")

	// Create a flagset for clean command to check for help
	cleanFlags := flag.NewFlagSet("clean", flag.ExitOnError)
	helpRequested := cleanFlags.Bool("help", false, "Show help for clean command")
	cleanFlags.BoolVar(helpRequested, "h", false, "Show help for clean command")

	// Parse the flags
	_ = cleanFlags.Parse(args)

	// Check if help was requested
	if *helpRequested {
		printCleanHelp()
		return
	}

	err := commands.HandleClean()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	// Assert postcondition - the command should complete without error
	assert(err == nil, "HandleClean should not return an error")
}

func handleStashCommand(args []string) {
	// Assert preconditions
	assert(args != nil, "args should not be nil")

	// Create a flagset for stash command to check for help
	stashFlags := flag.NewFlagSet("stash", flag.ExitOnError)
	helpRequested := stashFlags.Bool("help", false, "Show help for stash command")
	stashFlags.BoolVar(helpRequested, "h", false, "Show help for stash command")

	// Parse the flags
	_ = stashFlags.Parse(args)

	// Check if help was requested
	if *helpRequested {
		printStashHelp()
		return
	}

	err := commands.HandleStash()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	// Assert postcondition - the command should complete without error
	assert(err == nil, "HandleStash should not return an error")
}

func handleApplyCommand(args []string) {
	// Assert preconditions
	assert(args != nil, "args should not be nil")

	// Parse flags for apply command
	applyFlags := flag.NewFlagSet("apply", flag.ExitOnError)
	force := applyFlags.Bool("force", false, "Overwrite existing AGENTS.md file without prompting for confirmation")
	applyFlags.BoolVar(force, "f", false, "Overwrite existing AGENTS.md file without prompting for confirmation")
	helpRequested := applyFlags.Bool("help", false, "Show help for apply command")
	applyFlags.BoolVar(helpRequested, "h", false, "Show help for apply command")

	applyFlags.Parse(args)

	// Check if help was requested
	if *helpRequested {
		printApplyHelp()
		return
	}

	err := commands.HandleApply(*force)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	// Assert postcondition - the command should complete without error
	assert(err == nil, "HandleApply should not return an error")
}

func handleUninstallCommand(args []string) {
	// Assert preconditions
	assert(args != nil, "args should not be nil")

	// Create a flagset for uninstall command to check for help
	uninstallFlags := flag.NewFlagSet("uninstall", flag.ExitOnError)
	helpRequested := uninstallFlags.Bool("help", false, "Show help for uninstall command")
	uninstallFlags.BoolVar(helpRequested, "h", false, "Show help for uninstall command")

	// Parse the flags
	_ = uninstallFlags.Parse(args)

	// Check if help was requested
	if *helpRequested {
		printUninstallHelp()
		return
	}

	err := commands.HandleUninstall()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	// Assert postcondition - the command should complete without error
	assert(err == nil, "HandleUninstall should not return an error")
}
