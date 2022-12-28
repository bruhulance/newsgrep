use tokio;
mod website;
use website::Website;

#[tokio::main]
async fn main() {
    const URL: &str = "https://www.nytimes.com/ca/";
    let site =
        Website::with_url_and_sel(String::from(URL), String::from("section.story-wrapper > a"));

    // matching the return type of the response
    let res = match site.make_get().await {
        Ok(dump) => dump,
        Err(error) => {
            eprintln!("There has been an error: {:?}", error);
            String::new()
        }
    };

    // parsing the response to find articles
    match &site.find_articles(&res) {
        Ok(articles) => {
            println!("Found articles...");
            articles
                .iter()
                .for_each(|article| println!("article: {:?}", article.url));
        }
        Err(error) => eprintln!(
            "Something happened while trying to parse for articles, {:?}",
            error
        ),
    };
}
