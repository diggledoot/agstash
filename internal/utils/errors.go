package utils

import (
	"fmt"
)

// AgStashErrorType represents the type of error that occurred
type AgStashErrorType int

const (
	ProjectRootNotFound AgStashErrorType = iota
	HomeDirNotFound
	InvalidAgentsContent
	IoError
)

// AgStashError represents all possible errors that can occur in the agstash application
type AgStashError struct {
	Type    AgStashErrorType
	Message string
	Err     error
}

func (e *AgStashError) Error() string {
	return e.Message
}

func (e *AgStashError) Unwrap() error {
	return e.Err
}

// NewAgStashError creates a new AgStashError
func NewAgStashError(errorType AgStashErrorType, message string, err error) *AgStashError {
	return &AgStashError{
		Type:    errorType,
		Message: message,
		Err:     err,
	}
}

// NewProjectRootNotFoundError creates a new error for when project root is not found
func NewProjectRootNotFoundError() *AgStashError {
	return NewAgStashError(ProjectRootNotFound, "Could not find project root (no .git or .gitignore found)", nil)
}

// NewHomeDirNotFoundError creates a new error for when home directory is not found
func NewHomeDirNotFoundError() *AgStashError {
	return NewAgStashError(HomeDirNotFound, "Could not find home directory", nil)
}

// NewInvalidAgentsContentError creates a new error for when AGENTS.md content is invalid
func NewInvalidAgentsContentError(message string) *AgStashError {
	return NewAgStashError(InvalidAgentsContent, fmt.Sprintf("Invalid AGENTS content: %s", message), nil)
}

// NewIoError creates a new error for IO operations
func NewIoError(err error) *AgStashError {
	return NewAgStashError(IoError, fmt.Sprintf("IO error: %v", err), err)
}