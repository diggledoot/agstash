use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use log::{debug, info};
use std::fmt;
mod commands;
mod utils;

/// Custom error type for the agstash application
///
/// This enum represents all possible errors that can occur in the agstash application.
/// Each variant corresponds to a specific error condition that needs to be handled appropriately.
#[derive(Debug)]
pub enum AgStashError {
    /// Project root could not be found (no .git or .gitignore directory/file)
    ProjectRootNotFound,
    /// Home directory could not be found
    HomeDirNotFound,
    /// IO error occurred during file operations
    IoError(std::io::Error),
    /// AGENTS.md content is invalid (missing '# AGENTS' header)
    InvalidAgentsContent(String),
}

impl fmt::Display for AgStashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgStashError::ProjectRootNotFound => write!(
                f,
                "Could not find project root (no .git or .gitignore found)"
            ),
            AgStashError::HomeDirNotFound => write!(f, "Could not find home directory"),
            AgStashError::IoError(e) => write!(f, "IO error: {}", e),
            AgStashError::InvalidAgentsContent(msg) => write!(f, "Invalid AGENTS content: {}", msg),
        }
    }
}

impl std::error::Error for AgStashError {}

impl From<std::io::Error> for AgStashError {
    fn from(error: std::io::Error) -> Self {
        AgStashError::IoError(error)
    }
}

/// Command-line interface definition for the agstash application
///
/// This struct defines the CLI structure using clap, including all available commands
/// and their arguments. The verbose flag enables detailed logging output.
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None,
    styles = styles(),
    disable_help_subcommand = true
)]
struct Cli {
    /// Enable verbose logging for detailed output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

/// Available commands for the agstash application
///
/// These commands represent the core functionality of the agstash tool:
/// - Init: Create a new AGENTS.md file
/// - Clean: Remove the AGENTS.md file
/// - Stash: Store the AGENTS.md file in a global location
/// - Apply: Retrieve and apply a stashed AGENTS.md file
/// - Uninstall: Remove the global .agstash directory
#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new AGENTS.md file in the current directory
    Init,
    /// Remove the AGENTS.md file from the current directory
    Clean,
    /// Stash the AGENTS.md file to a global location for later retrieval
    Stash,
    /// Apply a previously stashed AGENTS.md file to the current directory
    Apply {
        /// Overwrite existing AGENTS.md file without prompting for confirmation
        #[arg(long)]
        force: bool,
    },
    /// Remove the global .agstash directory and all stashed files
    Uninstall,
}

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Cyan.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default())
}

/// Custom Result type that uses our AgStashError
///
/// This type alias makes it easier to work with our custom error type throughout the application.
type Result<T> = std::result::Result<T, AgStashError>;

/// Main entry point for the agstash application
///
/// This function handles:
/// 1. Parsing command-line arguments
/// 2. Setting up logging based on the verbose flag
/// 3. Executing the appropriate command based on user input
/// 4. Handling errors and exiting appropriately
fn main() {
    let cli = Cli::parse();

    // Initialize logging based on verbose flag
    // This allows users to get detailed output when needed for debugging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    info!("Starting agstash with command: {:?}", cli.command);
    debug!("Verbose mode enabled");

    // Execute the appropriate command handler based on user input
    // Each command is handled by a dedicated function in the commands module
    let result = match &cli.command {
        Commands::Init => {
            info!("Executing init command");
            commands::handle_init()
        }
        Commands::Clean => {
            info!("Executing clean command");
            commands::handle_clean()
        }
        Commands::Stash => {
            info!("Executing stash command");
            commands::handle_stash()
        }
        Commands::Apply { force } => {
            info!("Executing apply command with force: {}", force);
            commands::handle_apply(*force)
        }
        Commands::Uninstall => {
            info!("Executing uninstall command");
            commands::handle_uninstall()
        }
    };

    // Handle any errors that occurred during command execution
    // If an error occurred, print it to stderr and exit with code 1
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    info!("Command executed successfully");
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_agstash_error_display() {
        assert_eq!(
            format!("{}", AgStashError::ProjectRootNotFound),
            "Could not find project root (no .git or .gitignore found)"
        );
        assert_eq!(
            format!("{}", AgStashError::HomeDirNotFound),
            "Could not find home directory"
        );

        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        assert_eq!(
            format!("{}", AgStashError::IoError(io_error)),
            "IO error: file not found"
        );

        assert_eq!(
            format!("{}", AgStashError::InvalidAgentsContent("test".to_string())),
            "Invalid AGENTS content: test"
        );
    }
}
