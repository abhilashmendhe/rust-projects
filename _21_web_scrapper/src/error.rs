use thiserror::Error;


#[derive(Debug, Error)]
pub enum ScrapError {
    #[error("Request error: {0}")]
    RequestErr(#[from] reqwest::Error),

     #[error("Selector error: {0}")]
    SelectorErr(String)
}

impl<'a> From<scraper::error::SelectorErrorKind<'a>> for ScrapError {
    fn from(err: scraper::error::SelectorErrorKind<'a>) -> Self {
        ScrapError::SelectorErr(err.to_string())
    }
}