package utils

import (
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"
)

// SetupLogging configures the logging based on the verbose flag
func SetupLogging(verbose bool) {
	if !verbose {
		// If not verbose, suppress debug/info logs by setting output to io.Discard
		log.SetOutput(io.Discard)
	} else {
		// If verbose, log to standard output
		log.SetOutput(os.Stdout)
	}
}

// LogInfo logs an info message
func LogInfo(message string) {
	log.Printf("INFO: %s", message)
}

// LogWarn logs a warning message
func LogWarn(message string) {
	log.Printf("WARN: %s", message)
}

// IsValidAgents validates that the content starts with "# AGENTS"
func IsValidAgents(content string) bool {
	// For empty content, return false rather than panicking
	// This is more appropriate for validation functions that might legitimately receive empty input
	if content == "" {
		return false
	}

	// Check if content is too large to process safely
	if len(content) >= 10_000_000 {
		panic("Content too large to process safely")
	}

	return basicValidation(content)
}

func basicValidation(content string) bool {
	trimmedStart := strings.TrimLeft(content, " \t\n\r")
	return strings.HasPrefix(trimmedStart, "# AGENTS")
}

// GetProjectRoot finds the project root by looking for .git or .gitignore
func GetProjectRoot() (*AgStashError, string) {
	currentDir, err := os.Getwd()
	if err != nil {
		return NewIoError(err), ""
	}

	// Start from the current directory and work up
	currentPath := currentDir
	for {
		// Check if .git directory or .gitignore file exists
		gitDir := filepath.Join(currentPath, ".git")
		gitIgnoreFile := filepath.Join(currentPath, ".gitignore")

		if _, err := os.Stat(gitDir); err == nil {
			return nil, currentPath
		}
		if _, err := os.Stat(gitIgnoreFile); err == nil {
			return nil, currentPath
		}

		// Move up to parent directory
		parentPath := filepath.Dir(currentPath)
		// If we reached the root directory, break
		if parentPath == currentPath {
			break
		}
		currentPath = parentPath
	}

	return NewProjectRootNotFoundError(), ""
}

// GetStashPath returns the path where the project's AGENTS.md should be stashed
func GetStashPath(projectName string) (*AgStashError, string) {
	if projectName == "" {
		panic("Project name should not be empty")
	}

	homeDir, err := os.UserHomeDir()
	if err != nil {
		return NewHomeDirNotFoundError(), ""
	}

	stashDir := filepath.Join(homeDir, ".agstash", "stashes")

	// Create the stash directory if it doesn't exist
	if err := os.MkdirAll(stashDir, 0755); err != nil {
		return NewIoError(err), ""
	}

	stashPath := filepath.Join(stashDir, fmt.Sprintf("stash-%s.md", projectName))
	return nil, stashPath
}

// GetAgstashDir returns the path to the global .agstash directory
func GetAgstashDir() (*AgStashError, string) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return NewHomeDirNotFoundError(), ""
	}

	agstashDir := filepath.Join(homeDir, ".agstash")
	return nil, agstashDir
}

// ReadFile reads the content of a file
func ReadFile(path string) (*AgStashError, string) {
	content, err := os.ReadFile(path)
	if err != nil {
		return NewIoError(err), ""
	}
	return nil, string(content)
}

// WriteFile writes content to a file
func WriteFile(path string, content string) *AgStashError {
	err := os.WriteFile(path, []byte(content), 0644)
	if err != nil {
		return NewIoError(err)
	}
	return nil
}

// FileExists checks if a file exists
func FileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

// CopyFile copies a file from source to destination
func CopyFile(src, dst string) *AgStashError {
	// Read the source file
	srcData, err := os.ReadFile(src)
	if err != nil {
		return NewIoError(err)
	}

	// Write to the destination file
	err = os.WriteFile(dst, srcData, 0644)
	if err != nil {
		return NewIoError(err)
	}

	return nil
}