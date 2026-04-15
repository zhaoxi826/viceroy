/*
 * // Copyright 2026 zhaoxi826
 * //
 * // Licensed under the Apache License, Version 2.0 (the "License");
 * // you may not use this file except in compliance with the License.
 * // You may obtain a copy of the License at
 * //
 * //     http://www.apache.org/licenses/LICENSE-2.0
 * //
 * // Unless required by applicable law or agreed to in writing, software
 * // distributed under the License is distributed on an "AS IS" BASIS,
 * // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * // See the License for the specific language governing permissions and
 * // limitations under the License.
 */

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use viceroy::manifest::skill::analysis::process_and_save_skill;
use viceroy::manifest::skill::model::SkillModel;
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
    /// Install a skill from a Git repository and parse it
    Install {
        /// The Git repository URL
        url: String,

        /// Subdirectory path inside the repo (default is root)
        #[arg(short = 'p', long, default_value = "")]
        path: String,

        /// Root cache directory to clone into
        #[arg(short = 'c', long, default_value = ".cache")]
        cache_dir: String,

        /// Output directory to move the final skill into
        #[arg(short = 'o', long)]
        output: Option<String>,
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
        Commands::Install { url, path, cache_dir, output } => {
            let mut final_url = url.clone();
            if !final_url.starts_with("http://") && !final_url.starts_with("https://") && !final_url.starts_with("git@") {
                final_url = format!("https://github.com/{}", final_url);
            }

            println!("Installing skill from {} into {}", final_url, cache_dir);
            let skill = SkillModel::install(final_url, cache_dir.clone(), path.clone(), output.clone());
            println!("Analyzing installed skill at {}", skill.skill_path);
            skill.analysis()?;
            println!("Done.");
        }
    }

    Ok(())
}
