use clap::{App, Arg};
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
    uid: String,
    title: String,
    arg: String,
    autocomplete: String,
    quicklookurl: String,
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
            uid: r.searchtext.clone(),
            title: r.searchtext.clone(),
            arg: r.searchtext.clone(),
            autocomplete: r.searchtext.clone(),
            quicklookurl: format!(
                "https://www.ldoceonline.com/search/english/direct/?q={}",
                r.searchtext
            ),
        })
        .collect();
    Output { items }
}

fn get_input() -> String {
    let matches = App::new("Longman Query")
        .version("0.0.1")
        .author("Ryan Li <conbas2019@gmail.com>")
        .about("Query From Longman Dictionary of Contemporary English Online")
        .arg(
            Arg::with_name("INPUT")
                .help("Text to query")
                .required(true)
                .index(1),
        )
        .get_matches();
    matches.value_of("INPUT").unwrap().to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let q = get_input();
    let url = format!(
        "https://www.ldoceonline.com/autocomplete/english/?q={}&contentType=application%2Fjson%3B+charset%3Dutf-8",
        q,
    );
    let client = reqwest::Client::new();
    let body: Body = client.get(&url).send().await?.json().await?;
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
