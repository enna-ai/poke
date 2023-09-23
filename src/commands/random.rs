use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
#[clap(about = "Generates a random Pokemon")]
pub struct Options {
    #[clap(help = "Name of Pokémon")]
    pub count: usize,
}

pub async fn handle(options: &Options) -> Result<(), Box<dyn Error>> {
    println!("Hello, {}", options.count);

    Ok(())
}