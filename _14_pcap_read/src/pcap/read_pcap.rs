use std::fs::File;
use std::io;
use std::io::*;
use chrono::prelude::*;

pub fn read_pfile(file_path: &str) -> io::Result<()> {

    let file = File::open(file_path)?;

    let mut reader = BufReader::new(file);
    let mut file_header = [0 as u8; 24];
    let _ = reader.read_exact(&mut file_header).unwrap();
    
    // println!("{:X?}",file_header);

    // Read packet header
    let mut ph_timestamp = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_timestamp).unwrap();
    
    let mut ph_timestamp_frac = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_timestamp_frac).unwrap();

    let mut ph_cap_pk_len = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_cap_pk_len).unwrap();

    let _cap_pk_len = u32::from_le_bytes(ph_cap_pk_len);
    let mut ph_org_pk_len = [0 as u8; 4];
    let _ = reader.read_exact(&mut ph_org_pk_len).unwrap(); 

    let dt = get_time_stamp(&file_header[0..4], &ph_timestamp, &ph_timestamp_frac);
    
    println!("UTC: {}",dt);

    // Now read the whole packet data
    // -------------------------------

    // Read Ethernet header                  (Data Link)
    let mut src_eth_addr = [0 as u8; 6];
    let _ = reader.read_exact(&mut src_eth_addr).unwrap();

    let mut dst_eth_addr = [0 as u8; 6];
    let _ = reader.read_exact(&mut dst_eth_addr).unwrap();

    let mut eth_type = [0 as u8; 2];
    let _ = reader.read_exact(&mut eth_type);
    println!("{:?}", eth_type);

    // Read Internet protocol header          (Network Link)
    let mut ip_head_ver = [0 as u8];
    let _ = reader.read_exact(&mut ip_head_ver);
    
    let mut diff_serv = [0 as u8];
    let _ = reader.read_exact(&mut diff_serv);

    let mut packet_len_bytes = [0 as u8; 2];
    let _ = reader.read_exact(&mut packet_len_bytes);

    let mut identific = [0 as u8; 2];
    let _ = reader.read_exact(&mut identific);

    let mut flags = [0 as u8; 2];
    let _ = reader.read_exact(&mut flags);

    let mut ttl = [0 as u8];
    let _ = reader.read_exact(&mut ttl);

    let mut protocol = [0 as u8];
    let _ = reader.read_exact(&mut protocol);

    let mut header_checksum = [0 as u8; 2];
    let _ = reader.read_exact(&mut header_checksum);

    let mut src_addr = [0 as u8; 4];
    let _ = reader.read_exact(&mut src_addr);
    println!("{:?}", src_addr);

    let mut dest_addr = [0 as u8; 4];
    let _ = reader.read_exact(&mut dest_addr);
    println!("{:?}", dest_addr);

    // Read UDP (segments) (Transport Layer)
    let mut src_port = [0 as u8; 2];
    let _ = reader.read_exact(&mut src_port);

    let mut dst_port = [0 as u8; 2];
    let _ = reader.read_exact(&mut dst_port);
    
    let mut seg_len_bytes = [0 as u8; 2];
    let _ = reader.read_exact(&mut seg_len_bytes);

    let mut seg_checksum = [0 as u8; 2];
    let _ = reader.read_exact(&mut seg_checksum);

    let seg_len = u16::from_be_bytes(seg_len_bytes);

    let data_payload_len = seg_len - 8;

    println!("Data Payload Size: {}", data_payload_len);
    
    // Read remaining payload
    let mut data_payload = Vec::with_capacity(data_payload_len as usize);
    let _ = reader.read_exact(&mut data_payload);
    

    Ok(())
}

fn get_time_stamp(magic_header: &[u8], ph_timestamp: &[u8; 4], ph_timestamp_frac: &[u8; 4]) -> String {

    let timestamp = u32::from_le_bytes(*ph_timestamp) as i64;
    let mut timestamp_frac = u32::from_le_bytes(*ph_timestamp_frac);
    if *magic_header == [212, 195, 178, 161] {
        // Seconds and Microseconds
        timestamp_frac *= 1000;
        
    }

    let dt = DateTime
                            ::from_timestamp(timestamp, timestamp_frac)
                            .unwrap()
                            .format("%Y-%m-%d %H:%M:%S%.9f");		
    dt.to_string()
}