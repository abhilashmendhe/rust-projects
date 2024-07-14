use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

//constants
const SHAZAM_SERVER_ADDR: &str = "127.0.0.1:8000";

pub fn echo_server() {
    // Starting
    println!("shazam starting {}",SHAZAM_SERVER_ADDR);

    // bind
    let listener = TcpListener::bind(SHAZAM_SERVER_ADDR).unwrap();

    // start
    println!("SHAZAM Listening on {}",SHAZAM_SERVER_ADDR);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();

    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received: {}",message);
    
    // write back the respone
    let mut new_message = String::new();
    new_message.push_str("You sent: ");
    new_message.push_str(&message);
    let _ = stream.write_all(new_message.as_bytes());
    println!("sent: {}",new_message);
}   