// https://www.scrapingcourse.com/

use scraper::{Html, Selector};

use crate::error::ScrapError;

pub mod error;

fn main() -> Result<(), ScrapError> {

    let url = "https://www.google.com";
    let data: reqwest::blocking::Response = reqwest::blocking::get(url)?;
    // println!("data:{:?}",data);

    let body_text = data.text()?;

    let doc = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = doc
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    // println!("{:?}",doc);
    for href in href_values {
        println!("{}",href);
    }
    Ok(())
}
