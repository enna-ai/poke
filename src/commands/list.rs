use clap::Parser;
use reqwest;
use std::error::Error;
use serde_json::{json, Value};

#[derive(Debug, Parser)]
#[clap(about = "List Pokémon by type")]
pub struct Options {
    #[clap(help = "Type of Pokémon")]
    pub types: String,
}

pub async fn handle(options: &Options) -> Result<(), Box<dyn Error>> {
    let url = format!("https://pokeapi.co/api/v2/type/{}", options.types.to_lowercase());
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let body_bytes = response.bytes().await?;
        let body_str = String::from_utf8(body_bytes.to_vec())?;

        let json_value: Value = serde_json::from_str(&body_str)?;

        let pokemon_list = match &json_value["pokemon"] {
            Value::Array(pokemon_array) => {
                pokemon_array
                    .iter()
                    .map(|pokemon| {
                        let name = pokemon["pokemon"]["name"].as_str().unwrap_or("N/A");
                        let sprite = pokemon["pokemon"]["url"].as_str().unwrap_or("N/A");
                        json!({
                            "name": name,
                            "sprite": sprite,
                        })
                    })
                    .collect::<Vec<Value>>()
            }
            _ => Vec::new(),
        };

        let result_json = json!({
            "pokemon_list": pokemon_list,
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