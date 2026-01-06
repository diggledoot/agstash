package tests

import (
	"os"
	"path/filepath"
	"testing"

	"agstash/internal/commands"
	"agstash/internal/utils"
)

func TestInitCreatesFile(t *testing.T) {
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
	err := commands.HandleInit()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check if AGENTS.md was created
	agentsFile := "AGENTS.md"
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
}

func TestInitDoesNotOverwrite(t *testing.T) {
	// Create a temporary directory and change to it
	tempDir := t.TempDir()
	originalDir, _ := os.Getwd()
	defer os.Chdir(originalDir)
	os.Chdir(tempDir)

	// Create a .git directory to establish project root
	if err := os.Mkdir(".git", 0755); err != nil {
		t.Fatal(err)
	}

	// Create an existing AGENTS.md file
	agentsFile := "AGENTS.md"
	existingContent := "Existing content"
	if err := utils.WriteFile(agentsFile, existingContent); err != nil {
		t.Fatal(err)
	}

	// Run init command
	initErr := commands.HandleInit()
	if initErr != nil {
		t.Fatalf("Expected no error, got %v", initErr)
	}

	// Check that the file still has the original content
	readErr, content := utils.ReadFile(agentsFile)
	if readErr != nil {
		t.Fatal(readErr)
	}
	if content != existingContent {
		t.Errorf("Expected content %s, got %s", existingContent, content)
	}
}

func TestCleanRemovesFile(t *testing.T) {
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
	err := commands.HandleClean()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check if AGENTS.md was removed
	if utils.FileExists(agentsFile) {
		t.Error("Expected AGENTS.md to be removed after clean")
	}
}

func TestCleanDoesNotErrorOnMissingFile(t *testing.T) {
	// Create a temporary directory and change to it
	tempDir := t.TempDir()
	originalDir, _ := os.Getwd()
	defer os.Chdir(originalDir)
	os.Chdir(tempDir)

	// Create a .git directory to establish project root
	if err := os.Mkdir(".git", 0755); err != nil {
		t.Fatal(err)
	}

	// Run clean command on non-existing file
	err := commands.HandleClean()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
}

func TestStashCreatesFile(t *testing.T) {
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
	agentsContent := "# AGENTS\n\n- some content\n"
	if err := utils.WriteFile(agentsFile, agentsContent); err != nil {
		t.Fatal(err)
	}

	// Run stash command
	err := commands.HandleStash()
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

func TestStashFailsWhenAgentsMissing(t *testing.T) {
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

	// Don't create AGENTS.md

	// Run stash command - should not error but should not stash
	err := commands.HandleStash()
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}

	// Check that no stash was created
	projectName := filepath.Base(tempDir)
	stashPath := filepath.Join(tempDir, ".agstash", "stashes", "stash-"+projectName+".md")
	if utils.FileExists(stashPath) {
		t.Error("Expected no stash to be created when AGENTS.md doesn't exist")
	}
}

func TestUninstallRemovesDirectory(t *testing.T) {
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
	err := commands.HandleUninstall()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Check if .agstash directory was removed
	if utils.FileExists(agstashDir) {
		t.Error("Expected .agstash directory to be removed after uninstall")
	}
}

func TestStashRejectsInvalidAgentsContent(t *testing.T) {
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
	invalidContent := "Some invalid content"
	if err := utils.WriteFile(agentsFile, invalidContent); err != nil {
		t.Fatal(err)
	}

	// Run stash command - should not error but should not stash
	err := commands.HandleStash()
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