use clap::Parser;
use reqwest;
use std::error::Error;
use serde_json::{json, Value};

#[derive(Debug, Parser)]
#[clap(about = "Search for a Pokémon by name")]
pub struct Options {
    #[clap(help = "Name of Pokémon")]
    pub name: String,
}

pub async fn handle(options: &Options) -> Result<(), Box<dyn Error>> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", options.name.to_lowercase());
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

        Ok(())
    } else {
        println!("Request failed with status code: {}", response.status());

        let error_msg = format!("Request failed with status code: {}", response.status());
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)))
    }
}