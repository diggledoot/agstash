package commands

import (
	"bufio"
	"fmt"
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
func HandleInit(force bool) error {
	// Assert preconditions
	utils.Assert("AGENTS.md" != "", "agentsFilePath should not be empty")

	agentsFilePath := "AGENTS.md"

	// Check if we need user confirmation
	needsConfirmation := utils.FileExists(agentsFilePath) && !force
	if needsConfirmation {
		// Prompt user for confirmation before overwriting
		fmt.Printf("\n%s %s already exists in the current directory.\n", colorString("WARNING:", Yellow+Bold), colorString("AGENTS.md", Bold))
		fmt.Printf("Do you want to replace it with a default version?\n")
		fmt.Printf("This action will permanently overwrite the current file.\n\n")
		fmt.Printf("Type 'yes' to confirm or 'no' to cancel [y/N]: ")

		userConfirmed, err := getUserConfirmation()
		if err != nil {
			return err
		}
		if !userConfirmed {
			utils.LogInfo("User declined to overwrite, aborting init")
			fmt.Printf("\nOperation cancelled. %s was not modified.\n", colorString("AGENTS.md", Bold))
			return nil
		} else {
			utils.LogInfo("User confirmed overwrite")
			fmt.Printf("\nConfirmed. Creating default %s...\n", colorString("AGENTS.md", Bold))
		}
	} else if utils.FileExists(agentsFilePath) {
		utils.LogInfo("No existing AGENTS.md or force is true, proceeding with init")
	}

	// Content to write to the AGENTS.md file - initialize with just the header for an empty template
	agentsContent := `# AGENTS


`

	// Assert content is valid before writing
	utils.Assert(agentsContent != "", "agentsContent should not be empty")

	if err := utils.WriteFile(agentsFilePath, agentsContent); err != nil {
		return err
	}
	utils.LogInfo("Created AGENTS.md file")
	fmt.Printf("%s AGENTS.md\n", colorString("Created", Green))

	// Assert postcondition - file should exist after init
	if !utils.FileExists(agentsFilePath) {
		utils.LogInfo("AGENTS.md does not exist after init (it may have existed already)")
	} else {
		utils.LogInfo("AGENTS.md exists after init")
	}

	return nil
}

// HandleClean removes the AGENTS.md file from the current directory if it exists
func HandleClean() error {
	// Assert preconditions
	utils.Assert("AGENTS.md" != "", "agentsFilePath should not be empty")

	agentsFilePath := "AGENTS.md"

	if utils.FileExists(agentsFilePath) {
		if err := os.Remove(agentsFilePath); err != nil {
			return utils.NewIoError(err)
		}
		utils.LogInfo("Removed AGENTS.md file")
		fmt.Printf("%s AGENTS.md\n", colorString("Removed", Red))
	} else {
		utils.LogInfo("AGENTS.md does not exist, nothing to remove")
		fmt.Printf("%s %s\n", colorString("AGENTS.md", Bold), colorString("does not exist.", Yellow))
	}

	// Assert postcondition - file should not exist after clean
	if utils.FileExists(agentsFilePath) {
		utils.LogWarn("AGENTS.md still exists after clean operation")
	} else {
		utils.LogInfo("AGENTS.md does not exist after clean (as expected)")
	}

	return nil
}

// HandleStash reads the AGENTS.md file from the project root and copies it to a global stash location
func HandleStash() error {
	err, root := utils.GetProjectRoot()
	if err != nil {
		return err
	}

	// Assert preconditions
	utils.Assert(root != "", "root directory should not be empty")

	utils.LogInfo(fmt.Sprintf("Found project root at: %s", root))

	projectName := filepath.Base(root)

	// Assert project name is valid
	utils.Assert(projectName != "", "projectName should not be empty")

	agentsPath := filepath.Join(root, "AGENTS.md")

	if !utils.FileExists(agentsPath) {
		utils.LogInfo(fmt.Sprintf("AGENTS.md does not exist in project root: %s", agentsPath))
		fmt.Printf("%s %s\n", colorString("AGENTS.md", Bold), colorString("does not exist in project root.", Yellow))
		return nil
	}

	err, agentsContent := utils.ReadFile(agentsPath)
	if err != nil {
		return err
	}

	if !utils.IsValidAgents(agentsContent) {
		utils.LogWarn("AGENTS.md content is invalid, stash aborted")
		fmt.Printf("%s %s\n", colorString("AGENTS.md content is invalid (missing '# AGENTS' header).", Yellow), colorString("Stash aborted.", Yellow))
		return nil
	}

	err, stashPath := utils.GetStashPath(projectName)
	if err != nil {
		return err
	}

	// Assert stash path is valid
	utils.Assert(stashPath != "", "stashPath should not be empty")

	utils.LogInfo(fmt.Sprintf("Stashing to path: %s", stashPath))
	if err := utils.CopyFile(agentsPath, stashPath); err != nil {
		return err
	}
	utils.LogInfo(fmt.Sprintf("AGENTS.md stashed for project: %s", projectName))
	fmt.Printf("%s AGENTS.md for %s\n", colorString("Stashed", Green), colorString(projectName, Bold))

	// Assert postcondition - stashed file should exist
	if !utils.FileExists(stashPath) {
		utils.LogWarn("Stash file does not exist after stash operation")
	} else {
		utils.LogInfo("Stash file exists after stash operation")
	}

	return nil
}

// HandleApply copies the stashed AGENTS.md file back to the project root
func HandleApply(force bool) error {
	err, root := utils.GetProjectRoot()
	if err != nil {
		return err
	}

	// Assert preconditions
	utils.Assert(root != "", "root directory should not be empty")

	utils.LogInfo(fmt.Sprintf("Found project root at: %s", root))
	projectName := filepath.Base(root)

	// Assert project name is valid
	utils.Assert(projectName != "", "projectName should not be empty")

	err, stashFilePath := utils.GetStashPath(projectName)
	if err != nil {
		return err
	}
	agentsMdFilePath := filepath.Join(root, "AGENTS.md")

	// Assert file paths are valid
	utils.Assert(stashFilePath != "", "stashFilePath should not be empty")
	utils.Assert(agentsMdFilePath != "", "agentsMdFilePath should not be empty")

	utils.LogInfo(fmt.Sprintf("Looking for stash at: %s", stashFilePath))

	// Check if stash exists first
	if !utils.FileExists(stashFilePath) {
		utils.LogInfo(fmt.Sprintf("No stash found for project: %s", projectName))
		fmt.Printf("No stash found for project %s\n", colorString(projectName, Bold))
		return nil
	}

	// Check if we need user confirmation
	needsConfirmation := utils.FileExists(agentsMdFilePath) && !force
	if needsConfirmation {
		utils.LogInfo("AGENTS.md exists and force is false, prompting user")
		fmt.Printf("\n%s %s already exists in the current directory.\n", colorString("WARNING:", Yellow+Bold), colorString("AGENTS.md", Bold))
		fmt.Printf("Do you want to replace it with the stashed version?\n")
		fmt.Printf("This action will permanently overwrite the current file.\n\n")
		fmt.Printf("Type 'yes' to confirm or 'no' to cancel [y/N]: ")

		userConfirmed, err := getUserConfirmation()
		if err != nil {
			return err
		}
		if !userConfirmed {
			utils.LogInfo("User declined to overwrite, aborting apply")
			fmt.Printf("\nOperation cancelled. %s was not modified.\n", colorString("AGENTS.md", Bold))
			return nil
		} else {
			utils.LogInfo("User confirmed overwrite")
			fmt.Printf("\nConfirmed. Applying stashed %s...\n", colorString("AGENTS.md", Bold))
		}
	} else {
		utils.LogInfo("No existing AGENTS.md or force is true, proceeding with apply")
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
	// Assert preconditions
	utils.Assert(stashFilePath != "", "stashFilePath should not be empty")
	utils.Assert(agentsMdFilePath != "", "agentsMdFilePath should not be empty")
	utils.Assert(projectName != "", "projectName should not be empty")
	utils.Assert(utils.FileExists(stashFilePath), "Stash file path should exist")

	utils.LogInfo(fmt.Sprintf("Reading stash content from: %s", stashFilePath))
	err, stashContent := utils.ReadFile(stashFilePath)
	if err != nil {
		return err
	}

	// Assert content is valid before applying
	utils.Assert(stashContent != "", "stashContent should not be empty")

	if !utils.IsValidAgents(stashContent) {
		utils.LogWarn("Stash content is invalid, apply aborted")
		fmt.Printf("%s %s\n", colorString("Stash content is invalid (missing '# AGENTS' header).", Yellow), colorString("Apply aborted.", Yellow))
		return nil
	}

	utils.LogInfo(fmt.Sprintf("Applying stash to: %s", agentsMdFilePath))
	if err := utils.CopyFile(stashFilePath, agentsMdFilePath); err != nil {
		return err
	}
	utils.LogInfo(fmt.Sprintf("AGENTS.md applied for project: %s", projectName))
	fmt.Printf("%s AGENTS.md for %s\n", colorString("Applied", Green), colorString(projectName, Bold))

	// Assert postcondition - applied file should exist
	if !utils.FileExists(agentsMdFilePath) {
		utils.LogWarn("Applied file does not exist after apply operation")
	} else {
		utils.LogInfo("Applied file exists after apply operation")
	}

	return nil
}

// HandleUninstall completely removes the .agstash directory and all its contents from the user's home directory
func HandleUninstall() error {
	err, agstashDir := utils.GetAgstashDir()
	if err != nil {
		return err
	}

	// Assert preconditions
	utils.Assert(agstashDir != "", "agstashDir should not be empty")

	utils.LogInfo(fmt.Sprintf("Located agstash directory at: %s", agstashDir))

	if utils.FileExists(agstashDir) {
		utils.LogInfo(fmt.Sprintf("Removing agstash directory: %s", agstashDir))
		if err := os.RemoveAll(agstashDir); err != nil {
			return utils.NewIoError(err)
		}
		utils.LogInfo("Successfully removed agstash directory")
		fmt.Printf("%s %s\n", colorString("Removed", Red), agstashDir)
	} else {
		utils.LogInfo(fmt.Sprintf("agstash directory does not exist: %s", agstashDir))
		fmt.Printf("%s %s\n", colorString(".agstash directory", Bold), colorString("does not exist.", Yellow))
	}

	// Assert postcondition - directory should not exist after uninstall
	if utils.FileExists(agstashDir) {
		utils.LogWarn("agstash directory still exists after uninstall operation")
	} else {
		utils.LogInfo("agstash directory does not exist after uninstall (as expected)")
	}

	return nil
}
