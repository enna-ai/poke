use clap::Parser;
use reqwest;
use std::error::Error;
use std::io::Write;
use serde_json::{json, Value};
use std::fs::{self, File};
use std::env;

#[derive(Debug, Parser)]
#[clap(about = "Export a Pokémon to a JSON file")]
pub struct Options {
    #[clap(help = "Name of Pokémon")]
    name: String,
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

        let current_dir = env::current_dir()?;

        let pokemon_dir = current_dir.join("pokemon");
        fs::create_dir_all(&pokemon_dir)?;

        let file_path = pokemon_dir.join(format!("{}.json", name));

        let mut file = File::create(&file_path)?;

        let json_str = serde_json::to_string_pretty(&result_json)?;
        file.write_all(json_str.as_bytes())?;

        println!("Exported Pokemon {} to: {:?}", name, file_path);
    } else {
        println!("Failed to fetch pokemon.");
    }

    Ok(())
}