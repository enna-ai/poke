use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
#[clap(about = "List pokemon by type")]
pub struct Options {
    #[clap(help = "Type of Pokémon")]
    pub types: Vec<String>,
}

pub async fn handle(options: &Options) -> Result<(), Box<dyn Error>> {
    if !options.types.is_empty() {
        print!("Listing Pokémon by types:");
        for type_name in &options.types {
            print!("Type: {}", type_name);
        }
    } else {
        eprint!("Error: At least one type must be provided.");
        std::process::exit(1);
    }

    Ok(())
}
