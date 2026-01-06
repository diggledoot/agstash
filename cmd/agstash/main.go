package main

import (
	"flag"
	"fmt"
	"os"

	"agstash/internal/commands"
)

func main() {
	// Parse the flags to get the command
	if len(os.Args) < 2 {
		printUsage()
		os.Exit(1)
	}

	// Extract command and subcommand args
	command := os.Args[1]
	subArgs := os.Args[2:]

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
	help := `Usage: agstash init

Initialize a new AGENTS.md file in the current directory if one doesn't exist.

The AGENTS.md file contains guidelines for AI agents working in the project.

Examples:
  agstash init                    # Create AGENTS.md in current directory
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
	// Create a flagset for init command to check for help
	initFlags := flag.NewFlagSet("init", flag.ExitOnError)
	helpRequested := initFlags.Bool("help", false, "Show help for init command")
	initFlags.BoolVar(helpRequested, "h", false, "Show help for init command")

	// Parse the flags - if there are issues with other flags, we'll handle them later
	_ = initFlags.Parse(args)

	// Check if help was requested
	if *helpRequested {
		printInitHelp()
		return
	}

	err := commands.HandleInit()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func handleCleanCommand(args []string) {
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
}

func handleStashCommand(args []string) {
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
}

func handleApplyCommand(args []string) {
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
}

func handleUninstallCommand(args []string) {
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
}
