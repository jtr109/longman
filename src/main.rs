use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp: HashMap<String, String> =
        reqwest::get("https://httpbin.org/ip").await?.json().await?;
    println!("{:#?}", resp);
    Ok(())
}
