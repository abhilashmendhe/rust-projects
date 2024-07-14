use reqwest::{blocking::{Client, ClientBuilder}, header, redirect::Policy};

fn main() {
    let http_client = Client::new();
    // 1. Get Data
    let http_result = http_client.get("https://www.google.com").send();
    if http_result.is_ok() {
        // println!("Body: {:#?}",http_result.ok().unwrap().text().unwrap());
    } else {
        println!("{:#?}",http_result.err());
    }
    // 2. Post Data
    // let post_result = http_client
    //                     .post("http://localhost:3000/send_data")
    //                     .body("{\"first_name\":\"Abhilash\"}")
    //                     .header("User-Agent","Abhilash Mendhe Unibas").send();
                
    // println!("{:#?}",post_result.ok().unwrap().text().unwrap());
    
    // 3. Redirect
    let redir_policy = Policy::limited(5);
    let http_client = ClientBuilder::new().redirect(redir_policy).build().ok().unwrap();
    let http_result = http_client.get("http://localhost:3000/weather").send().ok().unwrap().text().unwrap();

    println!("Weather app result: {:#?}",http_result);
}
