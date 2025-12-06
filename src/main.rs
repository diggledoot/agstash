use anyhow::Result;
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = styles(), disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new AGENTS.md file
    Init,
    /// Remove the AGENTS.md file
    Clean,
    /// Stash the AGENTS.md file globally
    Stash,
    /// Apply the stashed AGENTS.md file
    Apply,
    /// Remove the global .agstash directory
    Uninstall,
}

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Cyan.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default())
}

fn get_project_root() -> Result<std::path::PathBuf> {
    let mut current_dir = std::env::current_dir()?;
    loop {
        if current_dir.join(".git").exists() || current_dir.join(".gitignore").exists() {
            return Ok(current_dir);
        }
        if !current_dir.pop() {
            break;
        }
    }
    Ok(std::env::current_dir()?)
}

fn get_stash_path(project_name: &str) -> Result<std::path::PathBuf> {
    let home_dir =
        home::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let stash_dir = home_dir.join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    Ok(stash_dir.join(format!("stash-{}.md", project_name)))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            let path = std::path::Path::new("AGENTS.md");
            if path.exists() {
                println!("{} {}", "AGENTS.md".bold(), "already exists.".yellow());
            } else {
                std::fs::write(
                    path,
                    r#"# AGENTS

- be concise and factual.
- always test after changes are made.
- create tests after a new feature is added.
"#,
                )?;
                println!("{} AGENTS.md", "Created".green());
            }
        }
        Commands::Clean => {
            let path = std::path::Path::new("AGENTS.md");
            if path.exists() {
                std::fs::remove_file(path)?;
                println!("{} AGENTS.md", "Removed".red());
            } else {
                println!("{} {}", "AGENTS.md".bold(), "does not exist.".yellow());
            }
        }
        Commands::Stash => {
            let root = get_project_root()?;
            let project_name = root.file_name().unwrap_or_default().to_string_lossy();
            let agents_path = root.join("AGENTS.md");

            if !agents_path.exists() {
                println!(
                    "{} {}",
                    "AGENTS.md".bold(),
                    "does not exist in project root.".yellow()
                );
                return Ok(());
            }

            let stash_path = get_stash_path(&project_name)?;
            std::fs::copy(&agents_path, &stash_path)?;
            println!(
                "{} AGENTS.md for {}",
                "Stashed".green(),
                project_name.bold()
            );
        }
        Commands::Apply => {
            let root = get_project_root()?;
            let project_name = root.file_name().unwrap_or_default().to_string_lossy();
            let stash_path = get_stash_path(&project_name)?;

            if !stash_path.exists() {
                println!("No stash found for project {}", project_name.bold());
                return Ok(());
            }

            let agents_path = root.join("AGENTS.md");
            if agents_path.exists() {
                println!(
                    "{} {} already exists. Overwrite? [y/N]",
                    "Warning:".yellow().bold(),
                    "AGENTS.md".bold()
                );

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim().to_lowercase();

                if input != "y" {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            std::fs::copy(&stash_path, &agents_path)?;
            println!(
                "{} AGENTS.md for {}",
                "Applied".green(),
                project_name.bold()
            );
        }
        Commands::Uninstall => {
            let home_dir =
                home::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
            let agstash_dir = home_dir.join(".agstash");

            if agstash_dir.exists() {
                std::fs::remove_dir_all(&agstash_dir)?;
                println!("{} {}", "Removed".red(), agstash_dir.to_string_lossy());
            } else {
                println!(
                    "{} {}",
                    ".agstash directory".bold(),
                    "does not exist.".yellow()
                );
            }
        }
    }

    Ok(())
}
