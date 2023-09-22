use clap::Parser;
use reqwest;
use serde_json::{json, Value};

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(short, long)]
    pokemon: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", args.pokemon);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let body_bytes = response.bytes().await?;
        let body_str = String::from_utf8(body_bytes.to_vec())?;

        let json_value: Value = serde_json::from_str(&body_str)?;

        let name = json_value["name"].as_str().unwrap_or("N/A");
        let species = json_value["species"]["name"].as_str().unwrap_or("N/A");
        let sprite = json_value["sprites"]["front_default"].as_str().unwrap_or("N/A");

        let result_json = json!({
            "name": name,
            "species": species,
            "sprite": sprite,
        });

        let result = serde_json::to_string_pretty(&result_json)?;

        println!("{}", result);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
