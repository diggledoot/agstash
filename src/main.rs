use clap::Parser;

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "agstash")]
#[command(about = "A tool for stashing and managing AGENTS.md files", long_about = None)]
struct Args {
    #[arg(short, long, help = "Enable verbose output")]
    verbose: bool,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Initialize a new empty AGENTS.md template in the current directory
    Init {
        #[arg(short = 'f', long, help = "Overwrite existing AGENTS.md file without prompting for confirmation")]
        force: bool,
    },
    /// Remove the AGENTS.md file from the current directory
    Clean,
    /// Stash the AGENTS.md file to a global location for later retrieval
    Stash,
    /// Apply a previously stashed AGENTS.md file to the current directory
    Apply {
        #[arg(short = 'f', long, help = "Overwrite existing AGENTS.md file without prompting for confirmation")]
        force: bool,
    },
    /// Remove the global .agstash directory and all stashed files
    Uninstall,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    utils::setup_logging(args.verbose);
    
    match &args.command {
        Some(Commands::Init { force }) => {
            commands::handle_init(*force)?;
        }
        Some(Commands::Clean) => {
            commands::handle_clean()?;
        }
        Some(Commands::Stash) => {
            commands::handle_stash()?;
        }
        Some(Commands::Apply { force }) => {
            commands::handle_apply(*force)?;
        }
        Some(Commands::Uninstall) => {
            commands::handle_uninstall()?;
        }
        None => {
            // Print usage when no command is provided
            print_usage();
        }
    }
    
    Ok(())
}

fn print_usage() {
    let usage = r#"
Usage: agstash <command> [options]

Available Commands:
  init        Initialize a new empty AGENTS.md template in the current directory
  clean       Remove the AGENTS.md file from the current directory
  stash       Stash the AGENTS.md file to a global location for later retrieval
  apply       Apply a previously stashed AGENTS.md file to the current directory
  uninstall   Remove the global .agstash directory and all stashed files
  help        Show this help message
"#;
    println!("{}", usage);
}