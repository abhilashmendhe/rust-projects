use std::fmt::Display;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    pub name: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationArea {
    pub count: u16, 
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Area>
    // pub results: HashMap<String, String>
}

pub fn fetch_location_area(poke_api: String, http_client: Client, location_area: &mut LocationArea) {
    let resp = http_client.get(poke_api).send();
    match resp {
        Ok(resp) => {
            let content = resp.text().unwrap();
            *location_area = serde_json::from_str(&content).unwrap();
            println!("{}",location_area);
        },
        Err(err) => {
            println!("{:?}",err);
        }
    }
}

impl LocationArea {
    pub fn new() -> Self {
        Self {
            count: 0,
            next: None,
            previous: None, 
            results: vec![]
            // results: HashMap::new()
        }
    }
}

impl Display for LocationArea {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all_loc = String::new();
        for res in &self.results {
            all_loc.push_str(format!("{}\n",res.name).as_str());
        }

        write!(f, "{}", all_loc)
    }
}
