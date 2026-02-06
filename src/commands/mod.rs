use std::fs;
use std::path::Path;
use std::io::{self, Write};

use crate::utils;

// ANSI color codes
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BOLD: &str = "\x1b[1m";

// color_string applies ANSI color codes to a string
fn color_string(s: &str, color_code: &str) -> String {
    format!("{}{}{}", color_code, s, RESET)
}

// HandleInit creates a default AGENTS.md file in the current directory if one doesn't exist
pub fn handle_init(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let agents_file_path = Path::new("AGENTS.md");

    // Check if we need user confirmation
    let needs_confirmation = utils::file_exists(agents_file_path) && !force;
    if needs_confirmation {
        // Prompt user for confirmation before overwriting
        println!(
            "\n{} {} already exists in the current directory.",
            color_string("WARNING:", &format!("{}{}", YELLOW, BOLD)),
            color_string("AGENTS.md", BOLD)
        );
        println!("Do you want to replace it with a default version?");
        println!("This action will permanently overwrite the current file.\n");
        print!("Type 'yes' to confirm or 'no' to cancel [y/N]: ");
        io::stdout().flush()?; // Ensure the prompt is displayed

        let user_confirmed = get_user_confirmation()?;
        if !user_confirmed {
            utils::log_info("User declined to overwrite, aborting init");
            println!("\nOperation cancelled. {} was not modified.", color_string("AGENTS.md", BOLD));
            return Ok(());
        } else {
            utils::log_info("User confirmed overwrite");
            println!("\nConfirmed. Creating default {}...", color_string("AGENTS.md", BOLD));
        }
    } else if utils::file_exists(agents_file_path) {
        utils::log_info("No existing AGENTS.md or force is true, proceeding with init");
    }

    // Content to write to the AGENTS.md file - initialize with just the header for an empty template
    let agents_content = "# AGENTS\n\n\n";

    if let Some(error) = utils::write_file(agents_file_path, agents_content) {
        return Err(error);
    }
    utils::log_info("Created AGENTS.md file");
    println!("{} AGENTS.md", color_string("Created", GREEN));

    Ok(())
}

// HandleClean removes the AGENTS.md file from the current directory if it exists
pub fn handle_clean() -> Result<(), Box<dyn std::error::Error>> {
    let agents_file_path = Path::new("AGENTS.md");

    if utils::file_exists(agents_file_path) {
        fs::remove_file(agents_file_path)?;
        utils::log_info("Removed AGENTS.md file");
        println!("{} AGENTS.md", color_string("Removed", RED));
    } else {
        utils::log_info("AGENTS.md does not exist, nothing to remove");
        println!(
            "{} {}",
            color_string("AGENTS.md", BOLD),
            color_string("does not exist.", YELLOW)
        );
    }

    Ok(())
}

// HandleStash reads the AGENTS.md file from the project root and copies it to a global stash location
pub fn handle_stash() -> Result<(), Box<dyn std::error::Error>> {
    let root = utils::get_project_root()?;

    utils::log_info(&format!("Found project root at: {}", root.display()));

    let project_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Could not extract project name")?;

    let agents_path = root.join("AGENTS.md");

    if !utils::file_exists(&agents_path) {
        utils::log_info(&format!("AGENTS.md does not exist in project root: {}", agents_path.display()));
        println!(
            "{} {}",
            color_string("AGENTS.md", BOLD),
            color_string("does not exist in project root.", YELLOW)
        );
        return Ok(());
    }

    let (err, agents_content) = utils::read_file(&agents_path);
    if let Some(error) = err {
        return Err(error);
    }

    if !utils::is_valid_agents(&agents_content) {
        utils::log_warn("AGENTS.md content is invalid, stash aborted");
        println!(
            "{} {}",
            color_string("AGENTS.md content is invalid (missing '# AGENTS' header).", YELLOW),
            color_string("Stash aborted.", YELLOW)
        );
        return Ok(());
    }

    let stash_path = utils::get_stash_path(project_name)?;

    utils::log_info(&format!("Stashing to path: {}", stash_path.display()));
    if let Some(error) = utils::copy_file(&agents_path, &stash_path) {
        return Err(error);
    }
    utils::log_info(&format!("AGENTS.md stashed for project: {}", project_name));
    println!(
        "{} AGENTS.md for {}",
        color_string("Stashed", GREEN),
        color_string(project_name, BOLD)
    );

    Ok(())
}

// HandleApply copies the stashed AGENTS.md file back to the project root
pub fn handle_apply(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let root = utils::get_project_root()?;

    utils::log_info(&format!("Found project root at: {}", root.display()));
    let project_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Could not extract project name")?;

    let stash_file_path = utils::get_stash_path(project_name)?;
    let agents_md_file_path = root.join("AGENTS.md");

    utils::log_info(&format!("Looking for stash at: {}", stash_file_path.display()));

    // Check if stash exists first
    if !utils::file_exists(&stash_file_path) {
        utils::log_info(&format!("No stash found for project: {}", project_name));
        println!("No stash found for project {}", color_string(project_name, BOLD));
        return Ok(());
    }

    // Check if we need user confirmation
    let needs_confirmation = utils::file_exists(&agents_md_file_path) && !force;
    if needs_confirmation {
        utils::log_info("AGENTS.md exists and force is false, prompting user");
        println!(
            "\n{} {} already exists in the current directory.",
            color_string("WARNING:", &format!("{}{}", YELLOW, BOLD)),
            color_string("AGENTS.md", BOLD)
        );
        println!("Do you want to replace it with the stashed version?");
        println!("This action will permanently overwrite the current file.\n");
        print!("Type 'yes' to confirm or 'no' to cancel [y/N]: ");
        io::stdout().flush()?; // Ensure the prompt is displayed

        let user_confirmed = get_user_confirmation()?;
        if !user_confirmed {
            utils::log_info("User declined to overwrite, aborting apply");
            println!("\nOperation cancelled. {} was not modified.", color_string("AGENTS.md", BOLD));
            return Ok(());
        } else {
            utils::log_info("User confirmed overwrite");
            println!("\nConfirmed. Applying stashed {}...", color_string("AGENTS.md", BOLD));
        }
    } else {
        utils::log_info("No existing AGENTS.md or force is true, proceeding with apply");
    }

    // Validate and apply the stash
    apply_stash_content(&stash_file_path, &agents_md_file_path, project_name)
}

fn get_user_confirmation() -> Result<bool, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    // Accept various forms of "yes"
    if ["y", "yes", "ye", "yep", "yeah"].contains(&input.as_str()) {
        return Ok(true);
    }
    // Accept various forms of "no" or default to no
    if ["n", "no", "nope", ""].contains(&input.as_str()) {
        return Ok(false);
    }
    // If input doesn't match expected values, default to false (no)
    Ok(false)
}

// apply_stash_content validates the stashed content and copies it to the project's AGENTS.md file
fn apply_stash_content(
    stash_file_path: &Path,
    agents_md_file_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    utils::log_info(&format!("Reading stash content from: {}", stash_file_path.display()));
    let (err, stash_content) = utils::read_file(stash_file_path);
    if let Some(error) = err {
        return Err(error);
    }

    if !utils::is_valid_agents(&stash_content) {
        utils::log_warn("Stash content is invalid, apply aborted");
        println!(
            "{} {}",
            color_string("Stash content is invalid (missing '# AGENTS' header).", YELLOW),
            color_string("Apply aborted.", YELLOW)
        );
        return Ok(());
    }

    utils::log_info(&format!("Applying stash to: {}", agents_md_file_path.display()));
    if let Some(error) = utils::copy_file(stash_file_path, agents_md_file_path) {
        return Err(error);
    }
    utils::log_info(&format!("AGENTS.md applied for project: {}", project_name));
    println!(
        "{} AGENTS.md for {}",
        color_string("Applied", GREEN),
        color_string(project_name, BOLD)
    );

    Ok(())
}

// HandleUninstall completely removes the .agstash directory and all its contents from the user's home directory
pub fn handle_uninstall() -> Result<(), Box<dyn std::error::Error>> {
    let agstash_dir = utils::get_agstash_dir()?;

    utils::log_info(&format!("Located agstash directory at: {}", agstash_dir.display()));

    if utils::file_exists(&agstash_dir) {
        utils::log_info(&format!("Removing agstash directory: {}", agstash_dir.display()));
        fs::remove_dir_all(&agstash_dir)?;
        utils::log_info("Successfully removed agstash directory");
        println!("{} {}", color_string("Removed", RED), agstash_dir.display());
    } else {
        utils::log_info(&format!("agstash directory does not exist: {}", agstash_dir.display()));
        println!(
            "{} {}",
            color_string(".agstash directory", BOLD),
            color_string("does not exist.", YELLOW)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;
    use serial_test::serial;

    use crate::commands;
    use crate::utils;

    #[test]
    #[serial]
    fn test_handle_init() {
        // Create a temporary directory and change to it
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        
        // Ensure cleanup happens
        let _cleanup = defer::defer(|| {
            let _ = env::set_current_dir(&original_dir);
        });

        // Create a .git directory to establish project root
        fs::create_dir(".git").unwrap();

        // Run init command with force to bypass confirmation
        let result = commands::handle_init(true);
        assert!(result.is_ok());

        // Check if AGENTS.md was created
        let agents_file = temp_dir.path().join("AGENTS.md");
        assert!(agents_file.exists());

        // Read the content and verify it
        let content = fs::read_to_string(&agents_file).unwrap();
        let expected_content = "# AGENTS\n\n\n";
        assert_eq!(content, expected_content);

        // Try to init again - should overwrite with force=true
        let result = commands::handle_init(true);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_handle_clean() {
        // Create a temporary directory and change to it
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        
        // Ensure cleanup happens
        let _cleanup = defer::defer(|| {
            let _ = env::set_current_dir(&original_dir);
        });

        // Create a .git directory to establish project root
        fs::create_dir(".git").unwrap();

        // Create an AGENTS.md file
        let agents_file = "AGENTS.md";
        let agents_content = "# AGENTS\n\nTest content";
        fs::write(agents_file, agents_content).unwrap();

        // Verify the file exists
        assert!(Path::new(agents_file).exists());

        // Run clean command
        let result = commands::handle_clean();
        assert!(result.is_ok());

        // Check if AGENTS.md was removed
        assert!(!Path::new(agents_file).exists());

        // Try to clean again - should not error
        let result = commands::handle_clean();
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_handle_stash() {
        // Create a temporary directory and change to it
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        
        // Ensure cleanup happens
        let _cleanup = defer::defer(|| {
            let _ = env::set_current_dir(&original_dir);
        });

        // Create a .git directory to establish project root
        fs::create_dir(".git").unwrap();

        // Set up HOME environment variable to temp directory
        let original_home = env::var("HOME").unwrap_or_default();
        env::set_var("HOME", temp_dir.path());
        
        // Ensure cleanup happens
        let _cleanup_home = defer::defer(move || {
            if !original_home.is_empty() {
                env::set_var("HOME", original_home);
            }
        });

        // Create an AGENTS.md file with valid content
        let agents_file = "AGENTS.md";
        let agents_content = "# AGENTS\n\nTest content";
        fs::write(agents_file, agents_content).unwrap();

        // Run stash command
        let result = commands::handle_stash();
        assert!(result.is_ok());

        // Check if the file was stashed
        let project_name = temp_dir.path().file_name().unwrap().to_str().unwrap();
        let stash_path = dirs::home_dir()
            .unwrap()
            .join(".agstash")
            .join("stashes")
            .join(format!("stash-{}.md", project_name));
            
        assert!(stash_path.exists());

        // Read the stashed content and verify it
        let stashed_content = fs::read_to_string(&stash_path).unwrap();
        assert_eq!(stashed_content, agents_content);
    }

    #[test]
    #[serial]
    fn test_handle_stash_invalid_content() {
        // Create a temporary directory and change to it
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        
        // Ensure cleanup happens
        let _cleanup = defer::defer(|| {
            let _ = env::set_current_dir(&original_dir);
        });

        // Create a .git directory to establish project root
        fs::create_dir(".git").unwrap();

        // Set up HOME environment variable to temp directory
        let original_home = env::var("HOME").unwrap_or_default();
        env::set_var("HOME", temp_dir.path());
        
        // Ensure cleanup happens
        let _cleanup_home = defer::defer(move || {
            if !original_home.is_empty() {
                env::set_var("HOME", original_home);
            }
        });

        // Create an AGENTS.md file with invalid content (missing header)
        let agents_file = "AGENTS.md";
        let agents_content = "Invalid content without header";
        fs::write(agents_file, agents_content).unwrap();

        // Run stash command - should not error but should not stash
        let result = commands::handle_stash();
        assert!(result.is_ok());

        // Check that no stash was created
        let project_name = temp_dir.path().file_name().unwrap().to_str().unwrap();
        let stash_path = dirs::home_dir()
            .unwrap()
            .join(".agstash")
            .join("stashes")
            .join(format!("stash-{}.md", project_name));
            
        // The stash directory might still be created even if no file is stashed
        // So we check if the specific stash file exists
        assert!(!stash_path.exists());
    }

    #[test]
    #[serial]
    fn test_handle_uninstall() {
        // Create a temporary directory to use as HOME
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").unwrap_or_default();
        env::set_var("HOME", temp_dir.path());
        
        // Ensure cleanup happens
        let _cleanup_home = defer::defer(move || {
            if !original_home.is_empty() {
                env::set_var("HOME", original_home);
            }
        });

        // Create the .agstash directory with some content
        let agstash_dir = dirs::home_dir().unwrap().join(".agstash");
        fs::create_dir_all(&agstash_dir).unwrap();

        // Create a test file inside .agstash
        let test_file = agstash_dir.join("test.txt");
        fs::write(&test_file, "test").unwrap();

        // Verify the directory exists
        assert!(agstash_dir.exists());

        // Run uninstall command
        let result = commands::handle_uninstall();
        assert!(result.is_ok());

        // Check if .agstash directory was removed
        assert!(!agstash_dir.exists());

        // Try to uninstall again - should not error
        let result = commands::handle_uninstall();
        assert!(result.is_ok());
    }
}