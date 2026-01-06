package commands

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/fatih/color"

	"agstash/internal/utils"
)

// HandleInit creates a default AGENTS.md file in the current directory if one doesn't exist
func HandleInit() error {
	agentsFilePath := "AGENTS.md"

	if utils.FileExists(agentsFilePath) {
		fmt.Printf("%s %s\n", color.New(color.FgGreen).Sprint("AGENTS.md"), color.New(color.FgYellow).Sprint("already exists."))
		log.Printf("INFO: AGENTS.md already exists, skipping creation")
	} else {
		// Content to write to the AGENTS.md file
		agentsContent := `# AGENTS

- be concise and factual.
- always test after changes are made.
- create tests after a new feature is added.
`

		if err := utils.WriteFile(agentsFilePath, agentsContent); err != nil {
			return err
		}
		log.Printf("INFO: Created AGENTS.md file")
		fmt.Printf("%s AGENTS.md\n", color.New(color.FgGreen).Sprint("Created"))
	}
	return nil
}

// HandleClean removes the AGENTS.md file from the current directory if it exists
func HandleClean() error {
	agentsFilePath := "AGENTS.md"

	if utils.FileExists(agentsFilePath) {
		if err := os.Remove(agentsFilePath); err != nil {
			return utils.NewIoError(err)
		}
		log.Printf("INFO: Removed AGENTS.md file")
		fmt.Printf("%s AGENTS.md\n", color.New(color.FgRed).Sprint("Removed"))
	} else {
		log.Printf("INFO: AGENTS.md does not exist, nothing to remove")
		fmt.Printf("%s %s\n", color.New(color.Bold).Sprint("AGENTS.md"), color.New(color.FgYellow).Sprint("does not exist."))
	}
	return nil
}

// HandleStash reads the AGENTS.md file from the project root and copies it to a global stash location
func HandleStash() error {
	err, root := utils.GetProjectRoot()
	if err != nil {
		return err
	}

	log.Printf("INFO: Found project root at: %s", root)

	projectName := filepath.Base(root)

	agentsPath := filepath.Join(root, "AGENTS.md")

	if !utils.FileExists(agentsPath) {
		log.Printf("INFO: AGENTS.md does not exist in project root: %s", agentsPath)
		fmt.Printf("%s %s\n", color.New(color.Bold).Sprint("AGENTS.md"), color.New(color.FgYellow).Sprint("does not exist in project root."))
		return nil
	}

	err, agentsContent := utils.ReadFile(agentsPath)
	if err != nil {
		return err
	}

	if !utils.IsValidAgents(agentsContent) {
		log.Printf("WARN: AGENTS.md content is invalid, stash aborted")
		fmt.Printf("%s %s\n", color.New(color.FgYellow).Sprint("AGENTS.md content is invalid (missing '# AGENTS' header)."), color.New(color.FgYellow).Sprint("Stash aborted."))
		return nil
	}

	err, stashPath := utils.GetStashPath(projectName)
	if err != nil {
		return err
	}

	log.Printf("INFO: Stashing to path: %s", stashPath)
	if err := utils.CopyFile(agentsPath, stashPath); err != nil {
		return err
	}
	log.Printf("INFO: AGENTS.md stashed for project: %s", projectName)
	fmt.Printf("%s AGENTS.md for %s\n", color.New(color.FgGreen).Sprint("Stashed"), color.New(color.Bold).Sprint(projectName))
	return nil
}

// HandleApply copies the stashed AGENTS.md file back to the project root
func HandleApply(force bool) error {
	err, root := utils.GetProjectRoot()
	if err != nil {
		return err
	}

	log.Printf("INFO: Found project root at: %s", root)
	projectName := filepath.Base(root)

	err, stashFilePath := utils.GetStashPath(projectName)
	if err != nil {
		return err
	}
	agentsMdFilePath := filepath.Join(root, "AGENTS.md")

	log.Printf("INFO: Looking for stash at: %s", stashFilePath)

	// Check if stash exists first
	if !utils.FileExists(stashFilePath) {
		log.Printf("INFO: No stash found for project: %s", projectName)
		fmt.Printf("No stash found for project %s\n", color.New(color.Bold).Sprint(projectName))
		return nil
	}

	// Check if we need user confirmation
	needsConfirmation := utils.FileExists(agentsMdFilePath) && !force
	if needsConfirmation {
		log.Printf("INFO: AGENTS.md exists and force is false, prompting user")
		fmt.Printf("%s %s already exists. Overwrite? [y/N]\n", color.New(color.FgYellow).Add(color.Bold).Sprint("Warning:"), color.New(color.Bold).Sprint("AGENTS.md"))

		userConfirmed, err := getUserConfirmation()
		if err != nil {
			return err
		}
		if !userConfirmed {
			log.Printf("INFO: User declined to overwrite, aborting apply")
			fmt.Printf("Aborted.\n")
			return nil
		} else {
			log.Printf("INFO: User confirmed overwrite")
		}
	} else {
		log.Printf("INFO: No existing AGENTS.md or force is true, proceeding with apply")
	}

	// Validate and apply the stash
	return applyStashContent(stashFilePath, agentsMdFilePath, projectName)
}

func getUserConfirmation() (bool, error) {
	scanner := bufio.NewScanner(os.Stdin)
	if scanner.Scan() {
		input := strings.TrimSpace(strings.ToLower(scanner.Text()))
		return input == "y" || input == "yes", nil
	}
	return false, scanner.Err()
}

// applyStashContent validates the stashed content and copies it to the project's AGENTS.md file
func applyStashContent(stashFilePath, agentsMdFilePath, projectName string) error {
	if !utils.FileExists(stashFilePath) {
		panic("Stash file path should exist")
	}

	if projectName == "" {
		panic("Project name should not be empty")
	}

	log.Printf("INFO: Reading stash content from: %s", stashFilePath)
	err, stashContent := utils.ReadFile(stashFilePath)
	if err != nil {
		return err
	}

	if !utils.IsValidAgents(stashContent) {
		log.Printf("WARN: Stash content is invalid, apply aborted")
		fmt.Printf("%s %s\n", color.New(color.FgYellow).Sprint("Stash content is invalid (missing '# AGENTS' header)."), color.New(color.FgYellow).Sprint("Apply aborted."))
		return nil
	}

	log.Printf("INFO: Applying stash to: %s", agentsMdFilePath)
	if err := utils.CopyFile(stashFilePath, agentsMdFilePath); err != nil {
		return err
	}
	log.Printf("INFO: AGENTS.md applied for project: %s", projectName)
	fmt.Printf("%s AGENTS.md for %s\n", color.New(color.FgGreen).Sprint("Applied"), color.New(color.Bold).Sprint(projectName))
	return nil
}

// HandleUninstall completely removes the .agstash directory and all its contents from the user's home directory
func HandleUninstall() error {
	err, agstashDir := utils.GetAgstashDir()
	if err != nil {
		return err
	}

	log.Printf("INFO: Located agstash directory at: %s", agstashDir)

	if utils.FileExists(agstashDir) {
		log.Printf("INFO: Removing agstash directory: %s", agstashDir)
		if err := os.RemoveAll(agstashDir); err != nil {
			return utils.NewIoError(err)
		}
		log.Printf("INFO: Successfully removed agstash directory")
		fmt.Printf("%s %s\n", color.New(color.FgRed).Sprint("Removed"), agstashDir)
	} else {
		log.Printf("INFO: agstash directory does not exist: %s", agstashDir)
		fmt.Printf("%s %s\n", color.New(color.Bold).Sprint(".agstash directory"), color.New(color.FgYellow).Sprint("does not exist."))
	}
	return nil
}