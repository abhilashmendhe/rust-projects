use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PokemonDetails {
    pokemon: Pokemon
}

#[derive(Debug, Serialize, Deserialize)]
struct PokemonFromArea {
    name: String,
    pokemon_encounters: Vec<PokemonDetails>
}

pub fn search_pokemon(client: Client, loc_url: String) -> Vec<String> {

    let resp = client.get(loc_url).send().unwrap();
    let json_str = resp.text().unwrap();

    let pokemon_from_area: PokemonFromArea = serde_json::from_str(&json_str).unwrap();
    // println!("{:?}",pokemon_from_area);
    let mut pokemon = vec![];

    for pd in pokemon_from_area.pokemon_encounters {
        pokemon.push(pd.pokemon.name);
    }
    pokemon
}

// mt-coronet-5f