use std::path::PathBuf;

pub fn is_valid_agents(content: &str) -> bool {
    assert!(
        content.len() < 10_000_000,
        "Content too large to process safely"
    ); // Prevent potential memory issues
    basic_validation(content)
}

fn basic_validation(content: &str) -> bool {
    let trimmed_start = content.trim_start();
    assert!(
        trimmed_start.len() <= content.len(),
        "Trim should not increase length"
    );
    trimmed_start.starts_with("# AGENTS")
}

pub fn get_project_root() -> Result<PathBuf, crate::AgStashError> {
    let mut current_dir = std::env::current_dir()?;
    loop {
        if current_dir.join(".git").exists() || current_dir.join(".gitignore").exists() {
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
    let home_dir = get_home_dir().ok_or(crate::AgStashError::HomeDirNotFound)?;
    let stash_dir = home_dir.join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    Ok(stash_dir.join(format!("stash-{}.md", project_name)))
}

pub fn get_agstash_dir() -> Result<PathBuf, crate::AgStashError> {
    let home_dir = get_home_dir().ok_or(crate::AgStashError::HomeDirNotFound)?;
    Ok(home_dir.join(".agstash"))
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

        let result = get_stash_path("test project with spaces");  // This creates a valid filename
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
        assert!(matches!(result, Err(crate::AgStashError::ProjectRootNotFound)));

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }
}
