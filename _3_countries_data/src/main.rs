
use std::{fs, io::{self, Read}};

use serde::{Serialize, Deserialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug)]
struct CountryName {
    common: String,
    official: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Currency {
    name: String,
    symbol: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Country {
    name: CountryName,
    region: String,
    borders: Option<Vec<String>>,
    languages: Option<Map<String, String>>,
    alt_spellings: Vec<String>,
    independent: Option<bool>,
    status: String,
    un_member: bool,
    capital: Option<Vec<String>>,
    continents: Vec<String>,
    flag: String,
    population: u64,
    currencies: Option<Map<String,Currency>>
}

fn main() -> Result<(), serde_json::Error> {

    // 1. Read json string  
    let json_content = read_json_file().unwrap();
    let data: Vec<Country> = serde_json::from_str(&json_content)?;
    
    loop {
        println!("1. Enter country name to get information.");
        println!("2. Get countries from continent.");
        println!("3. Exit");
        println!("\nChoose one of the above option.");
        let mut s = String::new();
        let _ = io::stdin().read_line(&mut s).expect("Can't take input!!");
        let n = s.trim().parse::<i32>().unwrap();

        if n == 1 {
            let mut s = String::new();
            println!("Enter the name of the country to retrieve data.");
            let _ = io::stdin().read_line(&mut s);
            // s = s.trim().;
            for country in &data {
                if country.name.common.to_lowercase().contains(s.trim()) {
                    println!("{:#?}\n",country);
                }
                // println!();
            }
        } else if n == 2 {
            let mut s = String::new();
            println!("Enter the continent to list the countries.");
            let _ = io::stdin().read_line(&mut s);
            for country in &data {
                for cont in &country.continents {
                    if cont.to_lowercase().contains(s.trim()) {
                        println!("{:#?\n}",country);
                    }
                }
            }
        } else if n == 3 {
            println!("Thank you.");
            break;
        } else {
            println!("Invalid number!");
        }
        

    }
    // println!("{:#?}",data);
    Ok(())    
}

fn read_json_file() -> Result<String, io::Error> {
    let content = fs::read_to_string("countries.json")?;
    Ok(content)
}  