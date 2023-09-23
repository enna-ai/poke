use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
#[clap(about = "Export a Pokémon to a JSON file")]
pub struct Options {
    #[clap(help = "Name of Pokémon")]
    name: String,
}

pub async fn handle(options: &Options) -> Result<(), Box<dyn Error>> {
    println!("Hello, {}", options.name);

    Ok(())
}