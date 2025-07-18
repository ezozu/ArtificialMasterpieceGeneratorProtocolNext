// src/main.rs
/*
 * Main executable for ArtificialMasterpieceGeneratorProtocolNext
 */

use clap::Parser;
use artificialmasterpiecegeneratorprotocolnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMasterpieceGeneratorProtocolNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
