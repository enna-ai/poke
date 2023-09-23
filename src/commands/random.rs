use clap::Parser;
use reqwest;
use std::error::Error;
use serde_json::{json, Value};
use rand::{thread_rng, Rng};

#[derive(Debug, Parser)]
#[clap(about = "Generates a random Pokemon")]
pub struct Options;

pub async fn handle(_: &Options) -> Result<(), Box<dyn Error>> {
    let url = format!("https://pokeapi.co/api/v2/pokemon-species?limit=1");
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let body_bytes = response.bytes().await?;
        let body_str = String::from_utf8(body_bytes.to_vec())?;

        let json_value: Value = serde_json::from_str(&body_str)?;
        let total_count = json_value["count"].as_u64().unwrap_or(0) as usize;

        let random_pokemon_id = thread_rng().gen_range(1..=total_count);

        let pokemon_url = format!("https://pokeapi.co/api/v2/pokemon/{}", random_pokemon_id);
        let pokemon_response = reqwest::get(&pokemon_url).await?;

        if pokemon_response.status().is_success() {
            let pokemon_body_bytes = pokemon_response.bytes().await?;
            let pokemon_body_str = String::from_utf8(pokemon_body_bytes.to_vec())?;

            let pokemon_json_value: Value = serde_json::from_str(&pokemon_body_str)?;
            let name = pokemon_json_value["name"].as_str().unwrap_or("N/A");
            let species = pokemon_json_value["species"]["name"].as_str().unwrap_or("N/A");
            let sprite = pokemon_json_value["sprites"]["front_default"].as_str().unwrap_or("N/A");

            let result_json = json!({
                "name": name,
                "species": species,
                "sprite": sprite,
            });

            let result = serde_json::to_string_pretty(&result_json)?;

            println!("Random Pokemon: {}", result);
        } else {
            println!("Failed to fetch a random pokemon.");
        }
    } else {
        println!("Failed to fetch total count of pokemon.");
    }

    Ok(())
}