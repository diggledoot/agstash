package main

import (
	"flag"
	"fmt"
	"os"

	"agstash/internal/commands"
)

func main() {
	// Assert preconditions
	if len(os.Args) < 1 {
		fmt.Fprintf(os.Stderr, "Assertion failed: os.Args should always have at least one element (program name)\n")
		os.Exit(1)
	}

	// Parse the flags to get the command
	if len(os.Args) < 2 {
		printUsage()
		os.Exit(1)
	}

	// Extract command and subcommand args
	command := os.Args[1]
	subArgs := os.Args[2:]

	// Assert command is not empty
	if command == "" {
		fmt.Fprintf(os.Stderr, "Assertion failed: Command should not be empty at this point\n")
		os.Exit(1)
	}

	// Define command handlers map
	commandHandlers := map[string]func([]string){
		"init":      handleInitCommand,
		"clean":     handleCleanCommand,
		"stash":     handleStashCommand,
		"apply":     handleApplyCommand,
		"uninstall": handleUninstallCommand,
		"help":      func(_ []string) { printUsage() },
	}

	// Handle commands
	if handler, exists := commandHandlers[command]; exists {
		handler(subArgs)
	} else {
		fmt.Fprintf(os.Stderr, "Unknown command: %s\n", command)
		printUsage()
		os.Exit(1)
	}
}

func printUsage() {
	usage := `
Usage: agstash <command> [options]

Available Commands:
  init        Initialize a new empty AGENTS.md template in the current directory
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
Creates an empty template with just the '# AGENTS' header for customization.

When an AGENTS.md file already exists in the current directory, the command will prompt
for confirmation before overwriting it. You will be asked to type 'yes' to confirm.

Flags:
  -f, --force    Overwrite existing AGENTS.md file without prompting for confirmation

The AGENTS.md file contains guidelines for AI agents working in the project.

Examples:
  agstash init                    # Create empty AGENTS.md template in current directory
  agstash init --force            # Create empty AGENTS.md template without confirmation
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

// parseForceAndHelpFlags parses common force and help flags
func parseForceAndHelpFlags(commandName string, args []string) (*bool, *bool, *flag.FlagSet) {
	// Assert preconditions
	if args == nil {
		fmt.Fprintf(os.Stderr, "Assertion failed: args should not be nil\n")
		os.Exit(1)
	}

	// Parse flags for command
	cmdFlags := flag.NewFlagSet(commandName, flag.ExitOnError)
	force := cmdFlags.Bool("force", false, "Overwrite existing AGENTS.md file without prompting for confirmation")
	cmdFlags.BoolVar(force, "f", false, "Overwrite existing AGENTS.md file without prompting for confirmation")
	helpRequested := cmdFlags.Bool("help", false, "Show help for "+commandName+" command")
	cmdFlags.BoolVar(helpRequested, "h", false, "Show help for "+commandName+" command")

	err := cmdFlags.Parse(args)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error parsing flags: %v\n", err)
		os.Exit(1)
	}

	return force, helpRequested, cmdFlags
}

func handleInitCommand(args []string) {
	// Parse common flags
	force, helpRequested, _ := parseForceAndHelpFlags("init", args)

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
}

func handleCleanCommand(args []string) {
	// Parse common flags (only help for clean command)
	_, helpRequested, _ := parseForceAndHelpFlags("clean", args)

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
	// Parse common flags (only help for stash command)
	_, helpRequested, _ := parseForceAndHelpFlags("stash", args)

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
	// Parse common flags
	force, helpRequested, _ := parseForceAndHelpFlags("apply", args)

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
	// Parse common flags (only help for uninstall command)
	_, helpRequested, _ := parseForceAndHelpFlags("uninstall", args)

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
