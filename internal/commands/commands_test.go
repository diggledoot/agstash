package commands

import (
	"os"
	"path/filepath"
	"testing"

	"agstash/internal/utils"
)

func TestHandleInit(t *testing.T) {
	// Create a temporary directory and change to it
	tempDir := t.TempDir()
	originalDir, _ := os.Getwd()
	defer os.Chdir(originalDir)
	os.Chdir(tempDir)

	// Create a .git directory to establish project root
	if err := os.Mkdir(".git", 0755); err != nil {
		t.Fatal(err)
	}

	// Run init command
	err := HandleInit()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check if AGENTS.md was created
	agentsFile := filepath.Join(tempDir, "AGENTS.md")
	if !utils.FileExists(agentsFile) {
		t.Error("Expected AGENTS.md to be created")
	}

	// Read the content and verify it
	err2, content := utils.ReadFile(agentsFile)
	if err2 != nil {
		t.Fatal(err2)
	}

	expectedContent := `# AGENTS

- be concise and factual.
- always test after changes are made.
- create tests after a new feature is added.
`
	if content != expectedContent {
		t.Errorf("Expected content %s, got %s", expectedContent, content)
	}

	// Try to init again - should not overwrite
	err = HandleInit()
	if err != nil {
		t.Errorf("Expected no error on second init, got %v", err)
	}
}

func TestHandleClean(t *testing.T) {
	// Create a temporary directory and change to it
	tempDir := t.TempDir()
	originalDir, _ := os.Getwd()
	defer os.Chdir(originalDir)
	os.Chdir(tempDir)

	// Create a .git directory to establish project root
	if err := os.Mkdir(".git", 0755); err != nil {
		t.Fatal(err)
	}

	// Create an AGENTS.md file
	agentsFile := "AGENTS.md"
	agentsContent := "# AGENTS\n\nTest content"
	if err := utils.WriteFile(agentsFile, agentsContent); err != nil {
		t.Fatal(err)
	}

	// Verify the file exists
	if !utils.FileExists(agentsFile) {
		t.Error("Expected AGENTS.md to exist before clean")
	}

	// Run clean command
	err := HandleClean()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check if AGENTS.md was removed
	if utils.FileExists(agentsFile) {
		t.Error("Expected AGENTS.md to be removed after clean")
	}

	// Try to clean again - should not error
	err = HandleClean()
	if err != nil {
		t.Fatalf("Expected no error on second clean, got %v", err)
	}
}

func TestHandleStash(t *testing.T) {
	// Create a temporary directory and change to it
	tempDir := t.TempDir()
	originalDir, _ := os.Getwd()
	defer os.Chdir(originalDir)
	os.Chdir(tempDir)

	// Create a .git directory to establish project root
	if err := os.Mkdir(".git", 0755); err != nil {
		t.Fatal(err)
	}

	// Set up HOME environment variable to temp directory
	originalHome := os.Getenv("HOME")
	t.Setenv("HOME", tempDir)
	defer os.Setenv("HOME", originalHome)

	// Create an AGENTS.md file with valid content
	agentsFile := "AGENTS.md"
	agentsContent := "# AGENTS\n\nTest content"
	if err := utils.WriteFile(agentsFile, agentsContent); err != nil {
		t.Fatal(err)
	}

	// Run stash command
	err := HandleStash()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check if the file was stashed
	projectName := filepath.Base(tempDir)
	stashPath := filepath.Join(tempDir, ".agstash", "stashes", "stash-"+projectName+".md")
	if !utils.FileExists(stashPath) {
		t.Error("Expected AGENTS.md to be stashed")
	}

	// Read the stashed content and verify it
	err2, stashedContent := utils.ReadFile(stashPath)
	if err2 != nil {
		t.Fatal(err2)
	}
	if stashedContent != agentsContent {
		t.Errorf("Expected stashed content %s, got %s", agentsContent, stashedContent)
	}
}

func TestHandleStashInvalidContent(t *testing.T) {
	// Create a temporary directory and change to it
	tempDir := t.TempDir()
	originalDir, _ := os.Getwd()
	defer os.Chdir(originalDir)
	os.Chdir(tempDir)

	// Create a .git directory to establish project root
	if err := os.Mkdir(".git", 0755); err != nil {
		t.Fatal(err)
	}

	// Set up HOME environment variable to temp directory
	originalHome := os.Getenv("HOME")
	t.Setenv("HOME", tempDir)
	defer os.Setenv("HOME", originalHome)

	// Create an AGENTS.md file with invalid content (missing header)
	agentsFile := "AGENTS.md"
	agentsContent := "Invalid content without header"
	if err := utils.WriteFile(agentsFile, agentsContent); err != nil {
		t.Fatal(err)
	}

	// Run stash command - should not error but should not stash
	err := HandleStash()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check that no stash was created
	projectName := filepath.Base(tempDir)
	stashPath := filepath.Join(tempDir, ".agstash", "stashes", "stash-"+projectName+".md")
	if utils.FileExists(stashPath) {
		t.Error("Expected no stash to be created for invalid content")
	}
}

func TestHandleUninstall(t *testing.T) {
	// Create a temporary directory and set it as HOME
	tempDir := t.TempDir()
	originalHome := os.Getenv("HOME")
	t.Setenv("HOME", tempDir)
	defer os.Setenv("HOME", originalHome)

	// Create the .agstash directory with some content
	agstashDir := filepath.Join(tempDir, ".agstash")
	if err := os.MkdirAll(agstashDir, 0755); err != nil {
		t.Fatal(err)
	}

	// Create a test file inside .agstash
	testFile := filepath.Join(agstashDir, "test.txt")
	if err := os.WriteFile(testFile, []byte("test"), 0644); err != nil {
		t.Fatal(err)
	}

	// Verify the directory exists
	if !utils.FileExists(agstashDir) {
		t.Error("Expected .agstash directory to exist before uninstall")
	}

	// Run uninstall command
	err := HandleUninstall()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check if .agstash directory was removed
	if utils.FileExists(agstashDir) {
		t.Error("Expected .agstash directory to be removed after uninstall")
	}

	// Try to uninstall again - should not error
	err = HandleUninstall()
	if err != nil {
		t.Fatalf("Expected no error on second uninstall, got %v", err)
	}
}