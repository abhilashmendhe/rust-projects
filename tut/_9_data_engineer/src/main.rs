use chrono::NaiveDate;
use serde::{Deserialize, Serialize};


#[derive(Debug,Deserialize,Serialize)]
struct ReviewRaw {
    name: String, 
    location: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Rating")]
    rating: String, 
    #[serde(rename = "Review")]
    review: String,
    #[serde(rename = "Image_Links")]
    image_links: String
}

#[derive(Debug,Deserialize,Serialize)]
struct ReviewCorrect {
    name: String, 
    location: String,
    #[serde(rename = "Date")]
    date: NaiveDate,
    #[serde(rename = "Rating")]
    rating: String, 
    #[serde(rename = "Review")]
    review: String,
    #[serde(rename = "Image_Links")]
    image_links: String
}

fn main() {
    // Read a csv file into an array of IrisRow?
    let mut csv_reader = csv::Reader::from_path("reviews_data.csv").unwrap();
    let mut reviews = csv_reader
                                    .deserialize::<ReviewRaw>()
                                    .map(|r| r.unwrap())
                                    .collect::<Vec<ReviewRaw>>();

    println!("{:?}",reviews.len());
    reviews.iter().take(3).for_each(|r| println!("{:#?}",r));
}
