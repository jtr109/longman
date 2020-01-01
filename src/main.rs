use error_chain::error_chain;
use serde::{Deserialize, Serialize};

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        JSON(serde_json::error::Error);
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

#[derive(Debug, Serialize)]
struct Item {
    title: String,
}

#[derive(Debug, Serialize)]
struct Output {
    items: Vec<Item>,
}

fn convert(body: Body) -> Output {
    let items = body
        .results
        .into_iter()
        .map(|r| Item {
            title: r.searchtext,
        })
        .collect();
    Output { items }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let body: Body = client
        .get("https://www.ldoceonline.com/autocomplete/english/?q=test&contentType=application%2Fjson%3B+charset%3Dutf-8")
        .send()
        .await?
        .json()
        .await?;
    let output = convert(body);
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

#[test]
fn test_convert() {
    let body = Body {
        results: vec![
            SearchResult {
                searchtext: "foo".to_string(),
            },
            SearchResult {
                searchtext: "bar".to_string(),
            },
        ],
    };
    let output = convert(body);
    assert_eq!(output.items[0].title, "foo");
    assert_eq!(output.items[1].title, "bar");
}
