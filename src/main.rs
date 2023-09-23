pub(crate) mod commands;

use clap::Parser;
use std::error::Error;
use commands::{Commands, handle_command};

#[derive(Parser, Debug)]
#[clap(
    version = "1.0.0",
    name = "poke",
    author = "enna-ai",
    about = "A simple Poké CLI"
)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    handle_command(args.command).await?;

    Ok(())
}