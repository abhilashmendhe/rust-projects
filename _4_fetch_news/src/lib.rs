use std::collections::HashMap;

use serde::{Serialize, Deserialize};
// use std::collections::HashMap as Map;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsSource {
    id: String,
    name: String,
    description: String, 
    url: String,
    category: String,
    language: String,
    country: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewsSources {
    pub status: String,
    pub sources: Vec<NewsSource>
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Source {
    id: Option<String>,
    name: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Article {
    source: Source,
    author: Option<String>, 
    title: Option<String>, 
    description: Option<String>, 
    url: Option<String>, 
    url_to_image: Option<String>, 
    published_at: Option<String>, 
    content: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct News {
    pub status: String, 
    pub total_results: u64,
    pub articles: Vec<Article>
}