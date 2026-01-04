use std::path::PathBuf;

pub fn is_valid_agents(content: &str) -> bool {
    // Assert input validity - content should not be too large to process safely
    assert!(
        content.len() < 10_000_000,
        "Content too large to process safely"
    );

    // For empty content, return false rather than panicking
    // This is more appropriate for validation functions that might legitimately receive empty input
    if content.is_empty() {
        return false;
    }

    basic_validation(content)
}

fn basic_validation(content: &str) -> bool {
    // The caller ensures content is not empty, so we can proceed with validation
    let trimmed_start = content.trim_start();
    // Assert that trim operation is working as expected
    assert!(
        trimmed_start.len() <= content.len(),
        "Trim should not increase length"
    );

    // The function already returns a boolean value
    trimmed_start.starts_with("# AGENTS")
}

pub fn get_project_root() -> Result<PathBuf, crate::AgStashError> {
    let mut current_dir = std::env::current_dir()?;

    // Assert that the current directory exists before starting search
    assert!(
        current_dir.exists(),
        "Current directory should exist before searching for project root"
    );

    loop {
        // Assert that the directory we're checking exists
        assert!(current_dir.exists(), "Directory being checked should exist");

        if current_dir.join(".git").exists() || current_dir.join(".gitignore").exists() {
            // Assert that the found path is valid before returning
            assert!(
                current_dir.is_absolute(),
                "Project root should be an absolute path"
            );
            return Ok(current_dir);
        }
        if !current_dir.pop() {
            break;
        }
    }
    Err(crate::AgStashError::ProjectRootNotFound)
}

// Function to get home directory - allows for easier testing
fn get_home_dir() -> Option<PathBuf> {
    home::home_dir()
}

pub fn get_stash_path(project_name: &str) -> Result<PathBuf, crate::AgStashError> {
    // Assert input validity - project name should not be empty
    assert!(!project_name.is_empty(), "Project name should not be empty");

    let home_dir = get_home_dir().ok_or(crate::AgStashError::HomeDirNotFound)?;

    // The home directory path is retrieved from the system, so it should be valid
    // Note: home_dir might not exist yet, so we allow this

    let stash_dir = home_dir.join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;

    // Assert that the stash directory was created successfully
    assert!(
        stash_dir.exists(),
        "Stash directory should exist after creation"
    );

    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));

    // Assert output validity - ensure the path is valid
    assert!(
        !stash_path.as_os_str().is_empty(),
        "Stash path should not be empty"
    );

    Ok(stash_path)
}

pub fn get_agstash_dir() -> Result<PathBuf, crate::AgStashError> {
    let home_dir = get_home_dir().ok_or(crate::AgStashError::HomeDirNotFound)?;

    // The home directory path is retrieved from the system, so it should be valid
    // Note: home_dir might not exist yet, so we allow this

    let agstash_dir = home_dir.join(".agstash");

    // Assert output validity - ensure the path is valid
    assert!(
        !agstash_dir.as_os_str().is_empty(),
        "Agstash directory path should not be empty"
    );

    Ok(agstash_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{TempDir, tempdir};

    #[test]
    fn test_is_valid_agents() {
        // Valid cases
        assert!(is_valid_agents("# AGENTS"));
        assert!(is_valid_agents("# AGENTS\n"));
        assert!(is_valid_agents("  # AGENTS")); // Leading spaces
        assert!(is_valid_agents("# AGENTS\n\n- content"));

        // Invalid cases
        assert!(!is_valid_agents(""));
        assert!(!is_valid_agents("# AGENT")); // Wrong header
        assert!(!is_valid_agents("- content")); // No header
        assert!(!is_valid_agents(" # AGENT")); // Space before #
        assert!(!is_valid_agents("AGENTS")); // Missing #
    }

    #[test]
    fn test_get_stash_path_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = std::env::var("HOME").unwrap_or_default();

        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }

        let result = get_stash_path("test-project");
        assert!(result.is_ok());

        let expected_path = temp_dir
            .path()
            .join(".agstash")
            .join("stashes")
            .join("stash-test-project.md");
        assert_eq!(result.unwrap(), expected_path);

        assert!(temp_dir.path().join(".agstash").join("stashes").exists());

        // Clean up
        unsafe {
            std::env::set_var("HOME", original_home);
        }
    }

    #[test]
    fn test_get_agstash_dir() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = std::env::var("HOME").unwrap_or_default();

        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }

        let result = get_agstash_dir();
        assert!(result.is_ok());

        let expected_path = temp_dir.path().join(".agstash");
        assert_eq!(result.unwrap(), expected_path);

        // Clean up
        unsafe {
            std::env::set_var("HOME", original_home);
        }
    }

    #[test]
    #[should_panic(expected = "Content too large to process safely")]
    fn test_is_valid_agents_large_content_panics() {
        // Create a string larger than 10MB
        let large_content = "a".repeat(10_000_001); // 10MB + 1 character
        is_valid_agents(&large_content);
    }

    #[test]
    fn test_is_valid_agents_max_size_allowed() {
        // Create a string just under the limit to ensure it doesn't panic
        let max_size_content = "# AGENTS\n".to_string() + &"a".repeat(9_999_990); // Just under 10MB
        // This should not panic and should return true since it starts with "# AGENTS"
        assert!(is_valid_agents(&max_size_content));
    }

    #[test]
    fn test_get_stash_path_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = std::env::var("HOME").unwrap_or_default();

        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }

        // Test project names with special characters
        let result = get_stash_path("test-project-with-dashes");
        assert!(result.is_ok());

        let result = get_stash_path("test_project_with_underscores");
        assert!(result.is_ok());

        let result = get_stash_path("testProject123");
        assert!(result.is_ok());

        let result = get_stash_path("test project with spaces"); // This creates a valid filename
        assert!(result.is_ok());

        // Clean up
        unsafe {
            std::env::set_var("HOME", original_home);
        }
    }

    #[test]
    fn test_get_project_root_not_found() {
        // Create a temporary directory without .git or .gitignore
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to the temp directory (which doesn't have .git or .gitignore)
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = get_project_root();
        assert!(matches!(
            result,
            Err(crate::AgStashError::ProjectRootNotFound)
        ));

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }
}
