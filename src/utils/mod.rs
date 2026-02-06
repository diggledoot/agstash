use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// SetupLogging configures the logging based on the verbose flag
pub fn setup_logging(verbose: bool) {
    // In Rust, we could use the env_logger or similar crate for more sophisticated logging
    // For now, we'll just note that verbose mode is enabled/disabled
    if verbose {
        eprintln!("Verbose logging enabled");
    }
}

// LogInfo logs an info message
pub fn log_info(message: &str) {
    eprintln!("INFO: {}", message);
}

// LogWarn logs a warning message
pub fn log_warn(message: &str) {
    eprintln!("WARN: {}", message);
}

// IsValidAgents validates that the content starts with "# AGENTS"
pub fn is_valid_agents(content: &str) -> bool {
    // For empty content, return false rather than panicking
    if content.is_empty() {
        return false;
    }

    // Check if content is too large to process safely
    if content.len() >= 10_000_000 {
        panic!("Content too large to process safely");
    }

    basic_validation(content)
}

fn basic_validation(content: &str) -> bool {
    let trimmed_start = content.trim_start_matches(|c: char| c == ' ' || c == '\t' || c == '\n' || c == '\r');
    trimmed_start.starts_with("# AGENTS")
}

// GetProjectRoot finds the project root by looking for .git or .gitignore
pub fn get_project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut current_path = env::current_dir()?;

    loop {
        // Check if .git directory or .gitignore file exists
        let git_dir = current_path.join(".git");
        let git_ignore_file = current_path.join(".gitignore");

        if git_dir.is_dir() || git_ignore_file.is_file() {
            return Ok(current_path);
        }

        // Move up to parent directory
        match current_path.parent() {
            Some(parent) => {
                current_path = parent.to_path_buf();
            }
            None => {
                // We've reached the root directory
                break;
            }
        }
    }

    Err("Project root not found".into())
}

// GetStashPath returns the path where the project's AGENTS.md should be stashed
pub fn get_stash_path(project_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if project_name.is_empty() {
        panic!("Project name should not be empty");
    }

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let stash_dir = home_dir.join(".agstash").join("stashes");

    // Create the stash directory if it doesn't exist
    fs::create_dir_all(&stash_dir)?;

    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    Ok(stash_path)
}

// GetAgstashDir returns the path to the global .agstash directory
pub fn get_agstash_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let agstash_dir = home_dir.join(".agstash");
    Ok(agstash_dir)
}

// ReadFile reads the content of a file - returns (error, content)
pub fn read_file<P: AsRef<Path>>(path: P) -> (Option<Box<dyn std::error::Error>>, String) {
    match fs::read_to_string(path) {
        Ok(content) => (None, content),
        Err(e) => (Some(Box::new(e)), String::new()),
    }
}

// WriteFile writes content to a file - returns error
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Option<Box<dyn std::error::Error>> {
    match fs::write(path, content) {
        Ok(_) => None,
        Err(e) => Some(Box::new(e)),
    }
}

// FileExists checks if a file exists
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

// RemoveFile removes a file
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_file(path)?;
    Ok(())
}

// CopyFile copies a file from source to destination - returns error
pub fn copy_file<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) -> Option<Box<dyn std::error::Error>> {
    match fs::copy(src, dst) {
        Ok(_) => None,
        Err(e) => Some(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;
    use std::path::Path;
    use tempfile::TempDir;
    use serial_test::serial;
    use crate::utils;

    #[test]
    fn test_is_valid_agents() {
        // Valid cases
        assert!(utils::is_valid_agents("# AGENTS"));
        assert!(utils::is_valid_agents("# AGENTS\n"));
        assert!(utils::is_valid_agents("  # AGENTS")); // Leading spaces
        assert!(utils::is_valid_agents("# AGENTS\n\n- content"));

        // Invalid cases
        assert!(!utils::is_valid_agents(""));
        assert!(!utils::is_valid_agents("# AGENT")); // Wrong header
        assert!(!utils::is_valid_agents("- content")); // No header
        assert!(!utils::is_valid_agents(" # AGENT")); // Space before #
        assert!(!utils::is_valid_agents("AGENTS")); // Missing #
    }

    #[test]
    #[serial]
    fn test_get_stash_path() {
        // Create a temporary directory to use as home
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").unwrap_or_default();
        env::set_var("HOME", temp_dir.path());
        
        // Ensure cleanup happens
        let _cleanup_home = defer::defer(move || {
            if !original_home.is_empty() {
                env::set_var("HOME", original_home);
            }
        });

        // Test with a sample project name
        let project_name = "test-project";
        let stash_path_result = utils::get_stash_path(project_name);

        assert!(stash_path_result.is_ok());
        let stash_path = stash_path_result.unwrap();

        let expected_path = temp_dir.path().join(".agstash").join("stashes").join("stash-test-project.md");
        assert_eq!(stash_path, expected_path);

        // Check if the stash directory was created
        let stash_dir = temp_dir.path().join(".agstash").join("stashes");
        assert!(stash_dir.exists());
    }

    #[test]
    #[serial]
    fn test_get_agstash_dir() {
        // Create a temporary directory to use as home
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").unwrap_or_default();
        env::set_var("HOME", temp_dir.path());
        
        // Ensure cleanup happens
        let _cleanup_home = defer::defer(move || {
            if !original_home.is_empty() {
                env::set_var("HOME", original_home);
            }
        });

        let agstash_dir_result = utils::get_agstash_dir();

        assert!(agstash_dir_result.is_ok());
        let agstash_dir = agstash_dir_result.unwrap();

        let expected_path = temp_dir.path().join(".agstash");
        assert_eq!(agstash_dir, expected_path);
    }

    #[test]
    fn test_file_exists() {
        // Create a temporary file
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test.txt");
        fs::write(&temp_file, "test").unwrap();

        // Test existing file
        assert!(utils::file_exists(&temp_file));

        // Test non-existing file
        let non_existing_file = temp_dir.path().join("non-existing.txt");
        assert!(!utils::file_exists(&non_existing_file));
    }

    #[test]
    fn test_read_file() {
        // Create a temporary file
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test.txt");
        let content = "test content";
        fs::write(&temp_file, content).unwrap();

        let (err, read_content) = utils::read_file(&temp_file);
        assert!(err.is_none());
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test.txt");
        let content = "test content";

        let err = utils::write_file(&temp_file, content);
        assert!(err.is_none());

        // Verify the file was written correctly
        let (read_err, read_content) = utils::read_file(&temp_file);
        assert!(read_err.is_none());
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_copy_file() {
        // Create source file
        let temp_dir = TempDir::new().unwrap();
        let src_file = temp_dir.path().join("source.txt");
        let src_content = "source content";
        fs::write(&src_file, src_content).unwrap();

        // Create destination file path
        let dst_file = temp_dir.path().join("destination.txt");

        // Copy the file
        let err = utils::copy_file(&src_file, &dst_file);
        assert!(err.is_none());

        // Verify the destination file has the correct content
        let (read_err, dst_content) = utils::read_file(&dst_file);
        assert!(read_err.is_none());
        assert_eq!(dst_content, src_content);
    }

    #[test]
    #[should_panic(expected = "Content too large to process safely")]
    fn test_is_valid_agents_large_content_panics() {
        // Create a string larger than 10MB
        let large_content = "a".repeat(10_000_001); // 10MB + 1 character

        utils::is_valid_agents(&large_content);
    }

    #[test]
    fn test_is_valid_agents_max_size_allowed() {
        // Create a string just under the limit to ensure it doesn't panic
        let max_size_content = "# AGENTS\n".to_string() + &"a".repeat(9_999_990); // Just under 10MB

        // This should not panic and should return true since it starts with "# AGENTS"
        assert!(utils::is_valid_agents(&max_size_content));
    }
}