use regex::Regex;
use tokio;
mod website;
use website::{Article, Scrapeable, SearchTerms};

#[tokio::main]
async fn main() {
    const URL: &str = "https://www.nytimes.com/ca/";
    let site = Article::new(
        String::from(URL),
        SearchTerms::Words(Regex::new(r"pele").unwrap()),
        String::from("section.story-wrapper > a"),
    );

    // matching the return type of the response
    match site.get().await {
        Ok(body) => {
            // parsing the response to find articles
            match &site.find_articles(&body) {
                Ok(articles) => {
                    site.print_articles(articles);
                    println!("\nFiltering the articles...");

                    // filtering the articles
                    match &site.filter_by_url(articles) {
                        Some(filtered) => site.print_articles(filtered),
                        None => println!("Filtering resulted in no articles."),
                    }
                }
                Err(error) => eprintln!(
                    "Something happened while trying to parse for articles, {:?}",
                    error
                ),
            };
        }
        Err(error) => eprintln!("There has been an error: {:?}", error),
    }
}
