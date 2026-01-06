package commands

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"agstash/internal/utils"
)

// ANSI color codes
const (
	Reset  = "\033[0m"
	Red    = "\033[31m"
	Green  = "\033[32m"
	Yellow = "\033[33m"
	Blue   = "\033[34m"
	Bold   = "\033[1m"
)

// colorString applies ANSI color codes to a string
func colorString(s string, colorCode string) string {
	return colorCode + s + Reset
}

// HandleInit creates a default AGENTS.md file in the current directory if one doesn't exist
func HandleInit() error {
	agentsFilePath := "AGENTS.md"

	if utils.FileExists(agentsFilePath) {
		fmt.Printf("%s %s\n", colorString("AGENTS.md", Green), colorString("already exists.", Yellow))
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
		fmt.Printf("%s AGENTS.md\n", colorString("Created", Green))
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
		fmt.Printf("%s AGENTS.md\n", colorString("Removed", Red))
	} else {
		log.Printf("INFO: AGENTS.md does not exist, nothing to remove")
		fmt.Printf("%s %s\n", colorString("AGENTS.md", Bold), colorString("does not exist.", Yellow))
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
		fmt.Printf("%s %s\n", colorString("AGENTS.md", Bold), colorString("does not exist in project root.", Yellow))
		return nil
	}

	err, agentsContent := utils.ReadFile(agentsPath)
	if err != nil {
		return err
	}

	if !utils.IsValidAgents(agentsContent) {
		log.Printf("WARN: AGENTS.md content is invalid, stash aborted")
		fmt.Printf("%s %s\n", colorString("AGENTS.md content is invalid (missing '# AGENTS' header).", Yellow), colorString("Stash aborted.", Yellow))
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
	fmt.Printf("%s AGENTS.md for %s\n", colorString("Stashed", Green), colorString(projectName, Bold))
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
		fmt.Printf("No stash found for project %s\n", colorString(projectName, Bold))
		return nil
	}

	// Check if we need user confirmation
	needsConfirmation := utils.FileExists(agentsMdFilePath) && !force
	if needsConfirmation {
		log.Printf("INFO: AGENTS.md exists and force is false, prompting user")
		fmt.Printf("\n%s %s already exists in the current directory.\n", colorString("WARNING:", Yellow+Bold), colorString("AGENTS.md", Bold))
		fmt.Printf("Do you want to replace it with the stashed version?\n")
		fmt.Printf("This action will permanently overwrite the current file.\n\n")
		fmt.Printf("Type 'yes' to confirm or 'no' to cancel [y/N]: ")

		userConfirmed, err := getUserConfirmation()
		if err != nil {
			return err
		}
		if !userConfirmed {
			log.Printf("INFO: User declined to overwrite, aborting apply")
			fmt.Printf("\nOperation cancelled. %s was not modified.\n", colorString("AGENTS.md", Bold))
			return nil
		} else {
			log.Printf("INFO: User confirmed overwrite")
			fmt.Printf("\nConfirmed. Applying stashed %s...\n", colorString("AGENTS.md", Bold))
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
		// Accept various forms of "yes"
		if input == "y" || input == "yes" || input == "ye" || input == "yep" || input == "yeah" {
			return true, nil
		}
		// Accept various forms of "no" or default to no
		if input == "n" || input == "no" || input == "nope" || input == "" {
			return false, nil
		}
		// If input doesn't match expected values, default to false (no)
		return false, nil
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
		fmt.Printf("%s %s\n", colorString("Stash content is invalid (missing '# AGENTS' header).", Yellow), colorString("Apply aborted.", Yellow))
		return nil
	}

	log.Printf("INFO: Applying stash to: %s", agentsMdFilePath)
	if err := utils.CopyFile(stashFilePath, agentsMdFilePath); err != nil {
		return err
	}
	log.Printf("INFO: AGENTS.md applied for project: %s", projectName)
	fmt.Printf("%s AGENTS.md for %s\n", colorString("Applied", Green), colorString(projectName, Bold))
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
		fmt.Printf("%s %s\n", colorString("Removed", Red), agstashDir)
	} else {
		log.Printf("INFO: agstash directory does not exist: %s", agstashDir)
		fmt.Printf("%s %s\n", colorString(".agstash directory", Bold), colorString("does not exist.", Yellow))
	}
	return nil
}