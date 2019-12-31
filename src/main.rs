#[macro_use]
extern crate error_chain;

use std::collections::HashMap;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let resp: HashMap<String, String> =
        reqwest::get("https://httpbin.org/ip").await?.json().await?;
    println!("{:#?}", resp);
    Ok(())
}
