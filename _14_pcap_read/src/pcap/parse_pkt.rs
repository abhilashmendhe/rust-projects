use std::{fs::File, io::{BufReader, Read}};

use chrono::{DateTime, Datelike, NaiveDate, Utc};

use crate::pcap::market_data_structure::QuoteData;

#[allow(unused)]

pub fn parse_data_layer(reader: &mut BufReader<File>) -> bool {

    let mut src_eth_addr = [0 as u8; 6];
    let _ = reader.read_exact(&mut src_eth_addr);

    let mut dst_eth_addr = [0 as u8; 6];
    let _ = reader.read_exact(&mut dst_eth_addr);

    let mut eth_type = [0 as u8; 2];
    let _ = reader.read_exact(&mut eth_type);
    
    if src_eth_addr[0..3] == [1, 128, 194] ||  
        // src_eth_addr[0..3] == [1, 0, 94] ||
        src_eth_addr[0..3] == [0, 0, 94] || 
        src_eth_addr[0..3] == [255, 255, 255] {
        return true;
    }
    return false;
}

pub fn parse_network_layer(reader: &mut BufReader<File>) -> (u8, i64) {

    let mut ip_head_ver = [0 as u8];
    let _ = reader.read_exact(&mut ip_head_ver);
    
    let mut diff_serv = [0 as u8];
    let _ = reader.read_exact(&mut diff_serv);

    let mut packet_len_bytes = [0 as u8; 2];
    let _ = reader.read_exact(&mut packet_len_bytes);

    let packet_len = u16::from_be_bytes(packet_len_bytes);

    let mut identific = [0 as u8; 2];
    let _ = reader.read_exact(&mut identific);

    let mut flags = [0 as u8; 2];
    let _ = reader.read_exact(&mut flags);

    let mut ttl = [0 as u8];
    let _ = reader.read_exact(&mut ttl);

    let mut protocol_bytes = [0 as u8];
    let _ = reader.read_exact(&mut protocol_bytes);

    let protocol = u8::from_be_bytes(protocol_bytes);

    let mut header_checksum = [0 as u8; 2];
    let _ = reader.read_exact(&mut header_checksum);

    let mut src_addr = [0 as u8; 4];
    let _ = reader.read_exact(&mut src_addr);

    let mut dest_addr = [0 as u8; 4];
    let _ = reader.read_exact(&mut dest_addr);

    // println!("Protocol: {}",protocol);
    (protocol, packet_len as i64)
}


pub fn parse_transport_layer(reader: &mut BufReader<File>) -> i64 {

    let mut src_port = [0 as u8; 2];
    let _ = reader.read_exact(&mut src_port);

    let mut dst_port = [0 as u8; 2];
    let _ = reader.read_exact(&mut dst_port);
    
    let mut seg_len_bytes = [0 as u8; 2];
    let _ = reader.read_exact(&mut seg_len_bytes);

    let mut seg_checksum = [0 as u8; 2];
    let _ = reader.read_exact(&mut seg_checksum);

    let seg_len = u16::from_be_bytes(seg_len_bytes);
    
    seg_len as i64
}

pub fn parse_udp_data (reader: &mut BufReader<File>, pkt_date_time: DateTime<Utc>) -> (usize, QuoteData) {

    // Intialize quotedata struct
    let mut quote_data = QuoteData::new();

    quote_data.set_pkt_time(pkt_date_time);

    quote_data.set_year(pkt_date_time.year());
    quote_data.set_month(pkt_date_time.month());
    quote_data.set_day(pkt_date_time.day());

    let mut data_type_ascii = [0 as u8; 5];
    let _ = reader.read_exact(&mut data_type_ascii);

    let begin_data = String::from_utf8_lossy(&data_type_ascii);

    if begin_data != "B6034" {
        return (5, QuoteData::new());
    }    

    let mut issue_code = [0 as u8; 12];
    let _ = reader.read_exact(&mut issue_code);
    
    // println!("{}",String::from_utf8_lossy(&mut issue_code));
    quote_data.set_issue_code(String::from_utf8_lossy(&mut issue_code).to_string());

    let mut issue_seq_no = [0 as u8; 3];
    let _ = reader.read_exact(&mut issue_seq_no);

    let mut market_status_type = [0 as u8; 2];
    let _ = reader.read_exact(&mut market_status_type);

    let mut total_bid_quote_vol = [0 as u8; 7];
    let _  = reader.read_exact(&mut total_bid_quote_vol);

    // println!("{:X?}", String::from_utf8_lossy(&mut total_bid_quote_vol));

    for _ in 0..5 {

        let mut best_bid_price = [0 as u8; 5];
        let _ = reader.read_exact(&mut best_bid_price);

        let mut best_bid_quantity = [0 as u8; 7];
        let _ = reader.read_exact(&mut best_bid_quantity);

        let price_quantity = (String::from_utf8_lossy(&best_bid_price).to_string(), String::from_utf8_lossy(&best_bid_quantity).to_string());
        quote_data.set_bid(price_quantity);
    }
    // quote_data.reverse_bids();

    let mut total_ask_quote_vol = [0 as u8; 7];
    let _ = reader.read_exact(&mut total_ask_quote_vol);

    for _ in 0..5 {

        let mut best_ask_price = [0 as u8; 5];
        let _ = reader.read_exact(&mut best_ask_price);

        let mut best_ask_quantity = [0 as u8; 7];
        let _ = reader.read_exact(&mut best_ask_quantity);

        let price_quantity = (String::from_utf8_lossy(&best_ask_price).to_string(), String::from_utf8_lossy(&best_ask_quantity).to_string());
        quote_data.set_ask(price_quantity);        
    }

    let mut num_best_bid_valid_quote = [0 as u8; 5];
    let _ = reader.read_exact(&mut num_best_bid_valid_quote);

    for _ in 0..5 {
        let mut num_best_bid_quote = [0 as u8; 4];
        let _ = reader.read_exact(&mut num_best_bid_quote);
    }

    let mut num_best_ask_valid_quote = [0 as u8; 5];
    let _ = reader.read_exact(&mut num_best_ask_valid_quote);

    for _ in 0..5 {
        let mut num_best_ask_quote = [0 as u8; 4];
        let _ = reader.read_exact(&mut num_best_ask_quote);
    }

    let mut remain = [0 as u8; 8]; // HH MM SS uu
    let _ = reader.read_exact(&mut remain);

    let acc_time_str = String::from_utf8_lossy(&remain).to_string();

    let hh = acc_time_str[0..2].parse::<u32>().unwrap();
    let mm = acc_time_str[2..4].parse::<u32>().unwrap();
    let ss = acc_time_str[4..6].parse::<u32>().unwrap();
    let uu = acc_time_str[6..8].parse::<u32>().unwrap();
    
    let accept_time = NaiveDate::
                            from_ymd_opt(quote_data.get_year(), quote_data.get_month(), quote_data.get_day())
                            .unwrap()
                            .and_hms_micro_opt(hh, mm, ss, uu)
                            .unwrap()
                            .and_utc();
    quote_data.set_accept_time(accept_time);

    return (0, quote_data);
}