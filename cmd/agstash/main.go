package main

import (
	"flag"
	"fmt"
	"os"

	"agstash/internal/commands"
	"agstash/internal/utils"
)

var verbose bool

func main() {
	// Set up flags
	verboseFlag := flag.Bool("v", false, "Enable verbose logging for detailed output")
	flag.BoolVar(verboseFlag, "verbose", false, "Enable verbose logging for detailed output")

	// Parse the flags to get the command
	if len(os.Args) < 2 {
		printUsage()
		os.Exit(1)
	}

	// Extract command and subcommand args
	command := os.Args[1]
	subArgs := os.Args[2:]

	// Set up logging based on verbose flag
	utils.SetupLogging(*verboseFlag)

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
	usage := `agstash is a CLI tool to stash and apply changes to AGENTS.md

Usage: agstash [flags] <command> [command-flags] [command-args]

Commands:
  init        Initialize a new AGENTS.md file in the current directory
  clean       Remove the AGENTS.md file from the current directory
  stash       Stash the AGENTS.md file to a global location for later retrieval
  apply       Apply a previously stashed AGENTS.md file to the current directory
  uninstall   Remove the global .agstash directory and all stashed files
  help        Show this help message

Flags:
  -v, --verbose    Enable verbose logging for detailed output
`
	fmt.Println(usage)
}

func handleInitCommand(args []string) {
	// Parse flags for init command (none currently)
	initFlags := flag.NewFlagSet("init", flag.ExitOnError)
	initFlags.Parse(args)

	err := commands.HandleInit()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func handleCleanCommand(args []string) {
	// Parse flags for clean command (none currently)
	cleanFlags := flag.NewFlagSet("clean", flag.ExitOnError)
	cleanFlags.Parse(args)

	err := commands.HandleClean()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func handleStashCommand(args []string) {
	// Parse flags for stash command (none currently)
	stashFlags := flag.NewFlagSet("stash", flag.ExitOnError)
	stashFlags.Parse(args)

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

	applyFlags.Parse(args)

	err := commands.HandleApply(*force)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func handleUninstallCommand(args []string) {
	// Parse flags for uninstall command (none currently)
	uninstallFlags := flag.NewFlagSet("uninstall", flag.ExitOnError)
	uninstallFlags.Parse(args)

	err := commands.HandleUninstall()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}