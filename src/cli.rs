use crate::config::Config;
use crate::output::print_report;
use crate::scan::scan_local_path;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load().context("failed to load Maple config")?;

    run_with_config(cli, &config)
}

fn run_with_config(cli: Cli, config: &Config) -> Result<()> {
    match cli.command {
        Command::Scan { target, json } => {
            let report = scan_local_path(&target, &config.scan)
                .with_context(|| format!("failed to scan {}", target.display()))?;

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report)
                        .context("failed to serialize scan report")?
                );
            } else {
                print_report(&report);
            }
        }
        Command::Serve { target } => {
            println!(
                "Local web explorer is not implemented yet. Target: {}",
                target.display()
            );
        }
        Command::Pack { target } => {
            println!(
                "Context pack export is not implemented yet. Target: {}",
                target.display()
            );
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "maple")]
#[command(about = "Map, explain, and package any repository.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scan a local repository and print a structural summary.
    Scan {
        /// Local path to scan.
        #[arg(default_value = ".")]
        target: PathBuf,

        /// Print machine-readable JSON.
        #[arg(long)]
        json: bool,
    },

    /// Start the local web explorer.
    Serve {
        /// Local path to explore.
        #[arg(default_value = ".")]
        target: PathBuf,
    },

    /// Export an AI-ready context pack.
    Pack {
        /// Local path to package.
        #[arg(default_value = ".")]
        target: PathBuf,
    },
}
