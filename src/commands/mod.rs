use crate::{Result, utils};
use colored::Colorize;
use log::{info, warn};

/// Initialize a new AGENTS.md file
pub fn handle_init() -> Result<()> {
    let agents_file_path = std::path::Path::new("AGENTS.md");
    if agents_file_path.exists() {
        println!("{} {}", "AGENTS.md".bold(), "already exists.".yellow());
        info!("AGENTS.md already exists, skipping creation");
    } else {
        std::fs::write(
            agents_file_path,
            r#"# AGENTS

- be concise and factual.
- always test after changes are made.
- create tests after a new feature is added.
"#,
        )?;
        info!("Created AGENTS.md file");
        println!("{} AGENTS.md", "Created".green());
    }
    Ok(())
}

/// Remove the AGENTS.md file
pub fn handle_clean() -> Result<()> {
    let agents_file_path = std::path::Path::new("AGENTS.md");
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
pub fn handle_stash() -> Result<()> {
    let root = utils::get_project_root()?;
    info!("Found project root at: {:?}", root);
    let project_name = root.file_name().unwrap_or_default().to_string_lossy();
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
pub fn handle_apply(force: bool) -> Result<()> {
    let root = utils::get_project_root()?;
    info!("Found project root at: {:?}", root);
    let project_name = root.file_name().unwrap_or_default().to_string_lossy();
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

fn apply_stash_content(
    stash_file_path: &std::path::Path,
    agents_md_file_path: &std::path::Path,
    project_name: &str,
) -> Result<()> {
    info!("Reading stash content from: {:?}", stash_file_path);
    let stash_content = std::fs::read_to_string(stash_file_path)?;

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
pub fn handle_uninstall() -> Result<()> {
    let agstash_dir = utils::get_agstash_dir()?;
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
