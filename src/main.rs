#[macro_use]
extern crate error_chain;
extern crate serde;

use serde::Deserialize;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
    }
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    searchtext: String,
}

#[derive(Debug, Deserialize)]
struct Body {
    results: Vec<SearchResult>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let resp: Body = client
        .get("https://www.ldoceonline.com/autocomplete/english/?q=test&contentType=application%2Fjson%3B+charset%3Dutf-8")
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
