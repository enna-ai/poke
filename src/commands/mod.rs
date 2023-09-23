mod search;
mod export;
mod list;
mod random;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Search(search::Options),
    Export(export::Options),
    List(list::Options),
    Random(random::Options),
}

pub use search::handle as search_handle;
pub use export::handle as export_handle;
pub use list::handle as list_handle;
pub use random::handle as random_handle;

pub async fn handle_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Search(options) => search_handle(&options).await,
        Commands::Export(options) => export_handle(&options).await,
        Commands::List(options) => list_handle(&options).await,
        Commands::Random(options) => random_handle(&options).await,
    }
}
