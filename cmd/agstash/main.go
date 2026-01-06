package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"agstash/internal/commands"
	"agstash/internal/utils"
)

var verbose bool

// RootCmd represents the base command when called without any subcommands
var RootCmd = &cobra.Command{
	Use:   "agstash",
	Short: "A CLI tool to stash and apply changes to AGENTS.md",
	Long:  `A CLI tool to stash and apply changes to AGENTS.md`,
}

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize a new AGENTS.md file in the current directory",
	Long:  `Initialize a new AGENTS.md file in the current directory`,
	Run: func(cmd *cobra.Command, args []string) {
		err := commands.HandleInit()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
	},
}

// cleanCmd represents the clean command
var cleanCmd = &cobra.Command{
	Use:   "clean",
	Short: "Remove the AGENTS.md file from the current directory",
	Long:  `Remove the AGENTS.md file from the current directory`,
	Run: func(cmd *cobra.Command, args []string) {
		err := commands.HandleClean()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
	},
}

// stashCmd represents the stash command
var stashCmd = &cobra.Command{
	Use:   "stash",
	Short: "Stash the AGENTS.md file to a global location for later retrieval",
	Long:  `Stash the AGENTS.md file to a global location for later retrieval`,
	Run: func(cmd *cobra.Command, args []string) {
		err := commands.HandleStash()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
	},
}

// applyCmd represents the apply command
var applyCmd = &cobra.Command{
	Use:   "apply",
	Short: "Apply a previously stashed AGENTS.md file to the current directory",
	Long:  `Apply a previously stashed AGENTS.md file to the current directory`,
	Run: func(cmd *cobra.Command, args []string) {
		force, _ := cmd.Flags().GetBool("force")
		err := commands.HandleApply(force)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
	},
}

// uninstallCmd represents the uninstall command
var uninstallCmd = &cobra.Command{
	Use:   "uninstall",
	Short: "Remove the global .agstash directory and all stashed files",
	Long:  `Remove the global .agstash directory and all stashed files`,
	Run: func(cmd *cobra.Command, args []string) {
		err := commands.HandleUninstall()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
	},
}

func init() {
	RootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "Enable verbose logging for detailed output")
	
	applyCmd.Flags().BoolP("force", "", false, "Overwrite existing AGENTS.md file without prompting for confirmation")
	
	RootCmd.AddCommand(initCmd)
	RootCmd.AddCommand(cleanCmd)
	RootCmd.AddCommand(stashCmd)
	RootCmd.AddCommand(applyCmd)
	RootCmd.AddCommand(uninstallCmd)
}

func main() {
	// Set up logging based on verbose flag
	utils.SetupLogging(verbose)
	
	if err := RootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}