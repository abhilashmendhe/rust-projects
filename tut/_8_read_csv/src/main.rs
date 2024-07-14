use std::process::exit;

use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Vehicle {
    #[serde(rename(deserialize = "Manufacturer"))]
    manufacturer: String,
    #[serde(rename(deserialize = "Model"))]
    model: String, 
    #[serde(rename(deserialize = "VIN"))]
    vin: String,
}

fn main() {
    let file_name = "data.csv";
    let mut builder = ReaderBuilder::new();
    builder.double_quote(false)
        .comment(Some(b'-'))
        .delimiter(b',')
        .has_headers(false);
    let result = builder.from_path(file_name);
    // let result = Reader::from_path(file_name);
    
    if result.is_err() {
        println!("Failed to read CSV file.");
        exit(9);
    }

    let mut reader = result.unwrap();
    // loop through records
    // for record in reader.records() {
    //     let car = record.unwrap();
    //     println!("Your car manufacturer is {:?}",car.get(0).unwrap());
    //     println!("Your car model is {:?}",car.get(1).unwrap());
    //     println!("Your car VIN is {:?}",car.get(2).unwrap());
    //     println!();
    // }

    // loop through records, derserializing
    for record in reader.deserialize() {
        let car: Vehicle = record.unwrap();
        println!("Your car manufacturer is {:?}",car.manufacturer);
        println!("Your car model is {:?}",car.model);
        println!("Your car VIN is {:?}",car.vin);
        println!();
    }
}
