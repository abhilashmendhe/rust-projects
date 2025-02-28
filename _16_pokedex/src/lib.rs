pub mod cache;
pub mod pokedex;
pub mod repl;
pub mod pokemons;

use std::io::{self, Write};

use cache::LocationArea;
use cache::fetch_location_area;
use pokemons::search_pokemon;
use repl::clean_input;

use reqwest::blocking::Client;


const _POKEAPI: &str = "https://pokeapi.co/api/v2/";

pub fn open_pokedex() {
    let http_client = Client::new();
    let mut _location_area = LocationArea::new();
    loop {
        print!("pokedex> ");
        io::stdout().flush().unwrap();

        // user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        
        let commands = clean_input(input);
        let first_command = &commands[0];
        println!("Your command was: {}",first_command);
        if first_command == "help" {
            println!("Welcome to the Pokedex!");
            println!("Usage:\n");
            println!("help: Display a help message");
            println!("exit: Exit the Pokedex");
        } else if first_command == "exit" {
            println!("Closing the Pokedex... Goodbye!");
            break;
        } else if first_command == "map" {
            let poke_api = format!("{}location-area",_POKEAPI);
            
            if _location_area.next.is_none() && _location_area.previous.is_none() {

                fetch_location_area(poke_api, 
                                    http_client.clone(), 
                                    &mut _location_area);
            } else {
                fetch_location_area(_location_area.next.clone().unwrap(), 
                                http_client.clone(), 
                                &mut _location_area);
            }
        } else if first_command == "mapb" {
            let poke_api = format!("{}location-area",_POKEAPI);
            // println!("{:?}",_location_area);
            if _location_area.next.is_none() && _location_area.previous.is_none() {

                fetch_location_area(poke_api, 
                                    http_client.clone(), 
                                    &mut _location_area);
            } else if _location_area.previous.is_none() {
                println!("You are already on the first page!");
            } else {
                fetch_location_area(_location_area.previous.clone().unwrap(), 
                                http_client.clone(), 
                                &mut _location_area);
            }
        } else if first_command == "explore" {
            if commands.len() > 1 {
                let loc_name_command = &commands[1];
                let a = _location_area
                                                    .results.iter()
                                                    .find(|area| *area.name == *loc_name_command)
                                                    .unwrap();
                let loc_url = a.url.clone();
                // println!("{}",loc_url);
                let all_pokemons = search_pokemon(http_client.clone(), loc_url);
                for poke in all_pokemons {
                    println!("{}",poke);
                }
            } else {
                println!("Please specify the location name to explore");
                println!("e.g. explore <location_name>")
            }
        }

        else {
            println!("Unknown command!");
        }
    }
}