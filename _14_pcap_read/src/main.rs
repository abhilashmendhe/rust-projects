/*
    https://wiki.wireshark.org/Development/LibpcapFileFormat
    https://www.ietf.org/archive/id/draft-gharris-opsawg-pcap-01.html#name-introduction
    https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
*/

use pcap::read_market::read_pfile;

mod pcap;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut args = std::env::args();
    let fpath = args.nth(1).unwrap();

    match std::env::args().nth(2) {
        Some(arg) => {
            if arg == "-r" {
                let market_data = read_pfile(&fpath, true)?;
                for data in market_data.1 {
                    println!("{}", data);
                }

            } else {
                println!("Not an correct argument. Please pass -r");
            }
        },
        None => {
            let market_data = read_pfile(&fpath, false)?;
            for data in market_data.0 {
                println!("{}", data);
            }
        }
    }
    Ok(())
}
