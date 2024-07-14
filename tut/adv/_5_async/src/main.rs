use std::time::{Duration, Instant};

use reqwest:: StatusCode;

async fn heartbeat(mut num: u32) {
    loop {
        println!("beating.... {}",num);
        tokio::time::sleep(Duration::from_millis(25)).await;
        num += 1;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    tokio::spawn(heartbeat(0));
    let (status_1, status_2) = tokio::join!(
        get_status("https://google.github.io/comprehensive-rust/index.html"),
        get_status("https://google.github.io/comprehensive-rust/types-and-values.html"));
    // let (status_1, status_2) = tokio::join!();
    println!("status 1: {:?}",status_1.unwrap());
    println!("status 2: {:?}",status_2.unwrap());
    println!("Overall execution time: {}ms", start_time.elapsed().as_millis());
    Ok(())
}

async fn get_status(url: &str) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let status_code = reqwest::get(url).await?.status();
    let duration = start_time.elapsed().as_millis();
    println!("Took {}ms to fetch url '{}'.",duration, url);
    Ok(status_code)
}

// 1. This runs serially. We are first waiting for the first task to complete. Then 2nd task is run
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let start_time = Instant::now();
//     let status_1 = get_status("https://google.github.io/comprehensive-rust/index.html").await?;
//     println!("status 1: {:?}",status_1);
//     let status_2 = get_status("https://google.github.io/comprehensive-rust/types-and-values.html").await?;
//     println!("status 2: {:?}",status_2);
//     println!("Overall execution time: {}ms", start_time.elapsed().as_millis());
//     Ok(())
// }

// async fn get_status(url: &str) -> Result<StatusCode, Box<dyn std::error::Error>> {
//     let start_time = Instant::now();

//     let status_code = reqwest::get(url).await?.status();
//     let duration = start_time.elapsed().as_millis();
//     println!("Took {}ms to fetch url '{}'.",duration, url);
//     Ok(status_code)
// }