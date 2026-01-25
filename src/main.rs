use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;

/// Terminal dashboard for Kata project visibility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to .planning directory (defaults to ./.planning)
    #[arg(short, long)]
    planning_dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Parse CLI arguments
    let args = Args::parse();

    // Run the application
    kata_tui::app::run(args.planning_dir).await
}
