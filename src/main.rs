mod dryrun;
mod process;
mod utils;

use std::{path::PathBuf, process::ExitCode};

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

    let result = match cli.command.unwrap_or(Commands::DryRun) {
        Commands::DryRun => {
            println!("Doing dryrun...");
            dryrun(&cli.path, cli.number)
        }
        Commands::Process => process(cli.path, cli.number),
    };

    if let Err(e) = result {
        eprintln!("final error was {:?}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
