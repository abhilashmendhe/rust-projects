use std::{fs::File, io::{BufReader, Error, Read}};

use chrono::{DateTime, Utc};

pub fn parse_headers(reader: &mut BufReader<File>, magic_header: &[u8]) -> Result<(DateTime<Utc>, i64), Error> {

    // Read packet header
    let mut ph_timestamp = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_timestamp)?;
    
    let mut ph_timestamp_frac = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_timestamp_frac)?;

    let mut ph_cap_pk_len = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_cap_pk_len)?;

    let cap_pk_len = u32::from_le_bytes(ph_cap_pk_len);
    
    let mut ph_org_pk_len = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_org_pk_len)?; 

    // println!("len of packets = {}", cap_pk_len);
    // println!("{}", u32::from_le_bytes(ph_org_pk_len));
    let dt = get_time_stamp(magic_header, &ph_timestamp, &ph_timestamp_frac);
    
    Ok((dt, cap_pk_len as i64))
}

fn get_time_stamp(magic_header: &[u8], ph_timestamp: &[u8; 4], ph_timestamp_frac: &[u8; 4]) ->  DateTime<Utc> {

    let timestamp = u32::from_le_bytes(*ph_timestamp) as i64;
    let mut timestamp_frac = u32::from_le_bytes(*ph_timestamp_frac);
    
    // println!("{} {}", timestamp, timestamp_frac);

    if *magic_header == [212, 195, 178, 161] {
        // Seconds and Microseconds
        timestamp_frac *= 1000;
        
    }

    let dt = DateTime
                            ::from_timestamp(timestamp, timestamp_frac)
                            .unwrap();           
    dt
}