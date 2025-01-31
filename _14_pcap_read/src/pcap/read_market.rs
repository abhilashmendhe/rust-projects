use std::collections::BTreeSet;
use std::fs::File;
use std::io;
use std::io::*;

use crate::pcap::parse_pkt_header::parse_headers;

use super::market_data_structure::QuoteData;
use super::parse_pkt::*;

pub fn read_pfile(file_path: &str, sort: bool) -> io::Result<(Vec<QuoteData>, BTreeSet<QuoteData>)> {

    let file = File::open(file_path)?;

    let mut _bin_tree: BTreeSet<QuoteData> = BTreeSet::new();

    let mut reader = BufReader::new(file);
    let mut file_header = [0 as u8; 24];
    let _ = reader.read_exact(&mut file_header).unwrap();
    
    let mut _x = 0;
    
    let magic_header = &file_header[0..4];
    let mut market_data = Vec::new();
    while let Ok((time_date, packet_len)) = parse_headers(&mut reader, magic_header) {

        _x+=1;
        // println!("{}) DateTime: {}.      Packet len: {}.",_x, time_date, packet_len);

        // Read Ethernet header                          (Data Link Layer)
        let is_stp_proto = parse_data_layer(&mut reader);
        if is_stp_proto {

            let _ = reader.seek_relative(packet_len - 14); // skip some bytes
            continue;
        }

        // Read Internet protocol header                  (Network Layer)
        let (protocol, _ip_pkt_len) = parse_network_layer(&mut reader);

        if protocol != 17 {
            let _ = reader.seek_relative(packet_len - 34);
            
        } else {


            // Read UDP (segments)                          (Transport Layer)
            let _transport_size = parse_transport_layer(&mut reader);
            
            // call parse market data function
            let (skip_size, _quote_data) = parse_udp_data(&mut reader, time_date);

            if skip_size > 0 {
                let _ = reader.seek_relative(packet_len - (42 + skip_size as i64));
            } else {
                // println!("{}", _quote_data);
                if sort {
                    _bin_tree.insert(_quote_data);
                } else {
                    market_data.push(_quote_data);
                }
                let _ = reader.seek_relative(1);
            }
        }
        // println!();
    }
    
    Ok((market_data, _bin_tree))
}

