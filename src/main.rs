// src/main.rs
/*
 * Main executable for UiProject
 */

use clap::Parser;
use uiproject::{Result, run};

#[derive(Parser)]
#[command(version, about = UiProject - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
