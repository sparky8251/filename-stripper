mod dryrun;
mod process;
mod utils;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use dryrun::dryrun;
use process::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, value_name = "DIRECTORY")]
    path: PathBuf,
    #[arg(short, long, value_name = "NUMBER TO REMOVE")]
    number: usize,
}

#[derive(Subcommand)]
enum Commands {
    DryRun,
    Process,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if !cli.path.is_dir() {
        eprintln!("path must be a directory!");
        return ExitCode::FAILURE;
    }

    match cli.command.unwrap_or(Commands::DryRun) {
        Commands::DryRun => {
            println!("Doing dryrun...");
            if dryrun(cli.path, cli.number).is_err() {
                return ExitCode::FAILURE;
            }
        }
        Commands::Process => {
            if process(cli.path, cli.number).is_err() {
                return ExitCode::FAILURE;
            }
        }
    };

    ExitCode::SUCCESS
}
