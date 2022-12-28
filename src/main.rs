use tokio;
mod website;
use website::Website;

#[tokio::main]
async fn main() {
    let mut site = Website::new();
    site.url = String::from("https://hyper.rs");

    site.make_get().await;
}

