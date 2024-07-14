use _4_fetch_news::{News, NewsSources};
use reqwest::{blocking::Client, header};


const API_KEY:&str = "78122f2957c443cca28bd4106eea0055";

fn main() {

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static(&API_KEY));
    let http_client = Client::builder().default_headers(headers).user_agent("cargo").build().unwrap();
     
    // 1. api to get the news sources
    // match http_client.get(format!("https://newsapi.org/v2/sources"))
    //             .send() {
    //     Ok(response) => {
    //         let content = response.text().unwrap();
    //         let sources: NewsSources = serde_json::from_str(&content).unwrap();

    //         println!("{:#?}",sources.status);
    //         // println!("{}",content);
    //     },
    //     Err(e) => {
    //         println!("{:?}",e);
    //     }
    // }

    // 2.     
    match http_client.get(format!("https://newsapi.org/v2/everything?q=bonds&q=finance&sortBy=popularity&language=en&pageSize=100"))
          .send() {
            Ok(response) => {
                let content = response.text().unwrap();
                // println!("{}",content);
                let news: News = serde_json::from_str(&content).unwrap();
                println!("{:#?}",news);
            },
            Err(e) => {
                println!("{:?}",e);
            }
    }
}
