package utils

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestIsValidAgents(t *testing.T) {
	// Valid cases
	if !IsValidAgents("# AGENTS") {
		t.Error("Expected '# AGENTS' to be valid")
	}
	if !IsValidAgents("# AGENTS\n") {
		t.Error("Expected '# AGENTS\\n' to be valid")
	}
	if !IsValidAgents("  # AGENTS") { // Leading spaces
		t.Error("Expected '  # AGENTS' to be valid")
	}
	if !IsValidAgents("# AGENTS\n\n- content") {
		t.Error("Expected '# AGENTS\\n\\n- content' to be valid")
	}

	// Invalid cases
	if IsValidAgents("") {
		t.Error("Expected empty string to be invalid")
	}
	if IsValidAgents("# AGENT") { // Wrong header
		t.Error("Expected '# AGENT' to be invalid")
	}
	if IsValidAgents("- content") { // No header
		t.Error("Expected '- content' to be invalid")
	}
	if IsValidAgents(" # AGENT") { // Space before #
		t.Error("Expected ' # AGENT' to be invalid")
	}
	if IsValidAgents("AGENTS") { // Missing #
		t.Error("Expected 'AGENTS' to be invalid")
	}
}

func TestBasicValidation(t *testing.T) {
	// This function is called by IsValidAgents, so we can test it indirectly
	if !basicValidation("# AGENTS") {
		t.Error("Expected '# AGENTS' to be valid")
	}
	if basicValidation("# AGENT") {
		t.Error("Expected '# AGENT' to be invalid")
	}
}

func TestGetStashPath(t *testing.T) {
	// Create a temporary directory to use as home
	tempDir := t.TempDir()
	t.Setenv("HOME", tempDir)

	// Test with a sample project name
	projectName := "test-project"
	err, stashPath := GetStashPath(projectName)

	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}

	expectedPath := filepath.Join(tempDir, ".agstash", "stashes", "stash-test-project.md")
	if stashPath != expectedPath {
		t.Errorf("Expected path %s, got %s", expectedPath, stashPath)
	}

	// Check if the stash directory was created
	stashDir := filepath.Join(tempDir, ".agstash", "stashes")
	if _, err := os.Stat(stashDir); os.IsNotExist(err) {
		t.Errorf("Expected stash directory %s to be created", stashDir)
	}
}

func TestGetAgstashDir(t *testing.T) {
	// Create a temporary directory to use as home
	tempDir := t.TempDir()
	t.Setenv("HOME", tempDir)

	err, agstashDir := GetAgstashDir()

	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}

	expectedPath := filepath.Join(tempDir, ".agstash")
	if agstashDir != expectedPath {
		t.Errorf("Expected path %s, got %s", expectedPath, agstashDir)
	}
}

func TestFileExists(t *testing.T) {
	// Create a temporary file
	tempFile := filepath.Join(t.TempDir(), "test.txt")
	if err := os.WriteFile(tempFile, []byte("test"), 0644); err != nil {
		t.Fatal(err)
	}

	// Test existing file
	if !FileExists(tempFile) {
		t.Errorf("Expected file %s to exist", tempFile)
	}

	// Test non-existing file
	nonExistingFile := filepath.Join(t.TempDir(), "non-existing.txt")
	if FileExists(nonExistingFile) {
		t.Errorf("Expected file %s to not exist", nonExistingFile)
	}
}

func TestReadFile(t *testing.T) {
	// Create a temporary file
	tempFile := filepath.Join(t.TempDir(), "test.txt")
	content := "test content"
	if err := os.WriteFile(tempFile, []byte(content), 0644); err != nil {
		t.Fatal(err)
	}

	err, readContent := ReadFile(tempFile)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
	if readContent != content {
		t.Errorf("Expected content %s, got %s", content, readContent)
	}
}

func TestWriteFile(t *testing.T) {
	tempFile := filepath.Join(t.TempDir(), "test.txt")
	content := "test content"

	err := WriteFile(tempFile, content)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}

	// Verify the file was written correctly
	_, readContent := ReadFile(tempFile)
	if readContent != content {
		t.Errorf("Expected content %s, got %s", content, readContent)
	}
}

func TestCopyFile(t *testing.T) {
	// Create source file
	srcFile := filepath.Join(t.TempDir(), "source.txt")
	srcContent := "source content"
	if err := os.WriteFile(srcFile, []byte(srcContent), 0644); err != nil {
		t.Fatal(err)
	}

	// Create destination file path
	dstFile := filepath.Join(t.TempDir(), "destination.txt")

	// Copy the file
	err := CopyFile(srcFile, dstFile)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Verify the destination file has the correct content
	err, dstContent := ReadFile(dstFile)
	if err != nil {
		t.Fatalf("Error reading destination file: %v", err)
	}
	if dstContent != srcContent {
		t.Errorf("Expected content %s, got %s", srcContent, dstContent)
	}
}

func TestIsValidAgentsLargeContentPanics(t *testing.T) {
	// Create a string larger than 10MB
	largeContent := strings.Repeat("a", 10_000_001) // 10MB + 1 character
	
	defer func() {
		if r := recover(); r == nil {
			t.Errorf("Expected panic for large content")
		} else if r != "Content too large to process safely" {
			t.Errorf("Expected panic message 'Content too large to process safely', got %v", r)
		}
	}()
	
	IsValidAgents(largeContent)
}

func TestIsValidAgentsMaxSizeAllowed(t *testing.T) {
	// Create a string just under the limit to ensure it doesn't panic
	maxSizeContent := "# AGENTS\n" + strings.Repeat("a", 9_999_990) // Just under 10MB
	
	// This should not panic and should return true since it starts with "# AGENTS"
	if !IsValidAgents(maxSizeContent) {
		t.Error("Expected content just under limit to be valid")
	}
}