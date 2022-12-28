use reqwest;
use tokio;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Some simple CLI args requirements...
    let url = String::from("https://hyper.rs");

    println!("Fetching {:?}...", url);

    // reqwest::get() is a convenience function.
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let res = reqwest::get(url).await?;

    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{}", body);

    Ok(())
}

