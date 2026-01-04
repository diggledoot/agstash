use crate::{Result, utils};
use colored::Colorize;
use log::{info, warn};

/// Initialize a new AGENTS.md file
///
/// This function creates a default AGENTS.md file in the current directory if one doesn't exist.
/// The file contains basic guidelines for AI agents working on the project.
pub fn handle_init() -> Result<()> {
    let agents_file_path = std::path::Path::new("AGENTS.md");

    // Assert that the path is valid
    assert!(
        !agents_file_path.as_os_str().is_empty(),
        "Agents file path should not be empty"
    );

    if agents_file_path.exists() {
        println!("{} {}", "AGENTS.md".bold(), "already exists.".yellow());
        info!("AGENTS.md already exists, skipping creation");
    } else {
        // Content to write to the AGENTS.md file
        let agents_content = r#"# AGENTS

- be concise and factual.
- always test after changes are made.
- create tests after a new feature is added.
"#;

        // The content is defined as a non-empty string literal, so no need to assert

        std::fs::write(agents_file_path, agents_content)?;
        info!("Created AGENTS.md file");
        println!("{} AGENTS.md", "Created".green());
    }
    Ok(())
}

/// Remove the AGENTS.md file
///
/// This function removes the AGENTS.md file from the current directory if it exists.
pub fn handle_clean() -> Result<()> {
    let agents_file_path = std::path::Path::new("AGENTS.md");

    // Assert that the path is valid
    assert!(
        !agents_file_path.as_os_str().is_empty(),
        "Agents file path should not be empty"
    );

    if agents_file_path.exists() {
        std::fs::remove_file(agents_file_path)?;
        info!("Removed AGENTS.md file");
        println!("{} AGENTS.md", "Removed".red());
    } else {
        info!("AGENTS.md does not exist, nothing to remove");
        println!("{} {}", "AGENTS.md".bold(), "does not exist.".yellow());
    }
    Ok(())
}

/// Stash the AGENTS.md file globally
///
/// This function reads the AGENTS.md file from the project root and copies it to a global stash location.
/// The stash location is determined by the project name and stored in the user's home directory.
pub fn handle_stash() -> Result<()> {
    let root = utils::get_project_root()?;

    // Assert that the project root is valid
    assert!(root.exists(), "Project root should exist");

    info!("Found project root at: {:?}", root);

    let project_name = root.file_name().unwrap_or_default().to_string_lossy();

    // Assert that project name is not empty
    assert!(!project_name.is_empty(), "Project name should not be empty");

    let agents_path = root.join("AGENTS.md");

    if !agents_path.exists() {
        info!(
            "AGENTS.md does not exist in project root: {:?}",
            agents_path
        );
        println!(
            "{} {}",
            "AGENTS.md".bold(),
            "does not exist in project root.".yellow()
        );
        return Ok(());
    }

    info!("Reading AGENTS.md content from: {:?}", agents_path);
    let agents_content = std::fs::read_to_string(&agents_path)?;

    // Assert that the content is not empty before validation
    assert!(
        !agents_content.is_empty(),
        "Agents content should not be empty before validation"
    );

    if !utils::is_valid_agents(&agents_content) {
        warn!("AGENTS.md content is invalid, stash aborted");
        println!(
            "{} {}",
            "AGENTS.md content is invalid (missing '# AGENTS' header).".yellow(),
            "Stash aborted.".yellow()
        );
        return Ok(());
    }

    let stash_path = utils::get_stash_path(&project_name)?;

    // The stash path is a valid PathBuf, so no need to assert existence
    // Path might not exist yet, which is fine

    info!("Stashing to path: {:?}", stash_path);
    std::fs::copy(&agents_path, &stash_path)?;
    info!("AGENTS.md stashed for project: {}", project_name);
    println!(
        "{} AGENTS.md for {}",
        "Stashed".green(),
        project_name.bold()
    );
    Ok(())
}

/// Apply the stashed AGENTS.md file
///
/// This function copies the stashed AGENTS.md file back to the project root.
/// If an AGENTS.md file already exists in the project and force is false, it prompts the user for confirmation.
pub fn handle_apply(force: bool) -> Result<()> {
    let root = utils::get_project_root()?;

    // Assert that the project root is valid
    assert!(root.exists(), "Project root should exist");

    info!("Found project root at: {:?}", root);
    let project_name = root.file_name().unwrap_or_default().to_string_lossy();

    // Assert that project name is not empty
    assert!(!project_name.is_empty(), "Project name should not be empty");

    let stash_file_path = utils::get_stash_path(&project_name)?;
    let agents_md_file_path = root.join("AGENTS.md");

    info!("Looking for stash at: {:?}", stash_file_path);

    // Check if stash exists first
    if !stash_file_path.exists() {
        info!("No stash found for project: {}", project_name);
        println!("No stash found for project {}", project_name.bold());
        return Ok(());
    }

    // Check if we need user confirmation
    let needs_confirmation = agents_md_file_path.exists() && !force;
    if needs_confirmation {
        info!("AGENTS.md exists and force is false, prompting user");
        println!(
            "{} {} already exists. Overwrite? [y/N]",
            "Warning:".yellow().bold(),
            "AGENTS.md".bold()
        );

        let user_confirmed = get_user_confirmation()?;
        if !user_confirmed {
            info!("User declined to overwrite, aborting apply");
            println!("Aborted.");
            return Ok(());
        } else {
            info!("User confirmed overwrite");
        }
    } else {
        info!("No existing AGENTS.md or force is true, proceeding with apply");
    }

    // Validate and apply the stash
    apply_stash_content(&stash_file_path, &agents_md_file_path, &project_name)
}

fn get_user_confirmation() -> Result<bool> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    Ok(input == "y")
}

/// Apply the stashed content to the project's AGENTS.md file
///
/// This function validates the stashed content and copies it to the project's AGENTS.md file.
/// It ensures the content is valid before applying it.
fn apply_stash_content(
    stash_file_path: &std::path::Path,
    agents_md_file_path: &std::path::Path,
    project_name: &str,
) -> Result<()> {
    // Assert input validity
    assert!(stash_file_path.exists(), "Stash file path should exist");

    assert!(!project_name.is_empty(), "Project name should not be empty");

    info!("Reading stash content from: {:?}", stash_file_path);
    let stash_content = std::fs::read_to_string(stash_file_path)?;

    // Assert that the content is not empty before validation
    assert!(
        !stash_content.is_empty(),
        "Stash content should not be empty before validation"
    );

    if !utils::is_valid_agents(&stash_content) {
        warn!("Stash content is invalid, apply aborted");
        println!(
            "{} {}",
            "Stash content is invalid (missing '# AGENTS' header).".yellow(),
            "Apply aborted.".yellow()
        );
        return Ok(());
    }

    info!("Applying stash to: {:?}", agents_md_file_path);
    std::fs::copy(stash_file_path, agents_md_file_path)?;
    info!("AGENTS.md applied for project: {}", project_name);
    println!(
        "{} AGENTS.md for {}",
        "Applied".green(),
        project_name.bold()
    );
    Ok(())
}

/// Remove the global .agstash directory
///
/// This function completely removes the .agstash directory and all its contents from the user's home directory.
/// This effectively uninstalls the agstash tool and removes all stashed AGENTS.md files.
pub fn handle_uninstall() -> Result<()> {
    let agstash_dir = utils::get_agstash_dir()?;

    // Assert that the agstash directory path is valid
    assert!(
        !agstash_dir.as_os_str().is_empty(),
        "Agstash directory path should not be empty"
    );

    info!("Located agstash directory at: {:?}", agstash_dir);

    if agstash_dir.exists() {
        info!("Removing agstash directory: {:?}", agstash_dir);
        std::fs::remove_dir_all(&agstash_dir)?;
        info!("Successfully removed agstash directory");
        println!("{} {}", "Removed".red(), agstash_dir.to_string_lossy());
    } else {
        info!("agstash directory does not exist: {:?}", agstash_dir);
        println!(
            "{} {}",
            ".agstash directory".bold(),
            "does not exist.".yellow()
        );
    }
    Ok(())
}
