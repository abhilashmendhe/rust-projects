use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

pub async fn echo_client_tokio() {
    println!("Connecting to {} .... ",ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        //  Connected
        println!("Connected to echo server {}:{}", 
                                    stream.local_addr().unwrap().ip(), 
                                    stream.local_addr().unwrap().port());

        // Write a hello world message
        let message = "Hello World!";
        let _ = stream.write_all(message.as_bytes()).await;
        // let _ = stream.flush();
        println!("sent: {}",message);

        // read the result
        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer).await;
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}",message);
    } else {
        println!("Failed to connect {}",ECHO_SERVER_ADDRESS);
    }
}