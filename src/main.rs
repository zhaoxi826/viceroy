use clap::{Parser, Subcommand};
use std::path::PathBuf;
use viceroy::manifest::skill::analysis::process_and_save_skill;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "viceroy")]
#[command(about = "Pretor's plugin management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a skill directory, extracting SKILL.md and analyzing python files.
    Parse {
        /// The path to the skill directory
        #[arg(short, long, value_name = "DIR")]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { path } => {
            if !path.exists() || !path.is_dir() {
                anyhow::bail!("Error: path {:?} does not exist or is not a directory", path);
            }
            println!("Parsing skill directory: {:?}", path);
            process_and_save_skill(path)?;
            println!("Done.");
        }
    }
    
    Ok(())
}
