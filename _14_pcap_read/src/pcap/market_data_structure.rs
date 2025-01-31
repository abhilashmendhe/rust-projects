
use std::fmt::Display;

use chrono::{DateTime, Utc};

type PRICE = String;
type QUANTITY = String;


#[derive(PartialEq, Eq, PartialOrd)]
pub struct QuoteData {
    pkt_time: Option<DateTime<Utc>>,
    accept_time: Option<DateTime<Utc>>,
    issue_code: String,
    best_bid: Vec<(PRICE, QUANTITY)>,
    best_ask: Vec<(PRICE, QUANTITY)>,
    year: i32,
    month: u32,
    day: u32
}

impl QuoteData {
    pub fn new() -> QuoteData {
        QuoteData {
            pkt_time: None,
            accept_time: None,
            issue_code: "".to_string(),
            best_bid: vec![],
            best_ask: vec![], 
            year: 0,
            month: 0,
            day: 0
        }
    }
    pub fn set_pkt_time(&mut self, date_time: DateTime<Utc>) {
        self.pkt_time = Some(date_time);
    }
    pub fn set_year(&mut self, year: i32) {
        self.year = year;
    }
    pub fn set_month(&mut self, month: u32) {
        self.month = month;
    }
    pub fn set_day(&mut self, day: u32) {
        self.day = day;
    }

    pub fn get_year(&self) -> i32 {
        self.year
    }
    pub fn get_month(&self) -> u32 {
        self.month
    }
    pub fn get_day(&self) -> u32 {
        self.day
    }
    
    pub fn set_accept_time(&mut self, date_time: DateTime<Utc>) {
        self.accept_time = Some(date_time);
    }
    pub fn set_issue_code(&mut self, issue_code: String) {
        self.issue_code = issue_code;
    }
    pub fn set_bid(&mut self, price_quantity: (PRICE, QUANTITY)) {
        self.best_bid.push(price_quantity);
    }
    pub fn set_ask(&mut self, price_quantity: (PRICE, QUANTITY)) {
        self.best_ask.push(price_quantity);
    }
}

impl Display for QuoteData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut s = String::new();
        s.push_str(&self.pkt_time.unwrap().to_string());
        s.push(',');
        s.push(' ');
        s.push_str(&self.accept_time.unwrap().to_string());
        s.push(',');
        s.push(' ');
        s.push_str(&self.issue_code);
        s.push(',');
        s.push(' ');

        // append best bid
        for best_bid in self.best_bid.clone().iter().rev() {
            s.push_str(format!("{}@{}",best_bid.0, best_bid.1).as_str());
            s.push(' ');
        }
        s.push(',');
        s.push(' ');

        // append best ask
        for best_ask in self.best_ask.clone() {
            s.push_str(format!("{}@{}",best_ask.0, best_ask.1).as_str());
            s.push(' ');
        }

        f.write_str(&s)
    }
}

impl Ord for QuoteData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.accept_time.unwrap().cmp(&other.accept_time.unwrap())
    }
}