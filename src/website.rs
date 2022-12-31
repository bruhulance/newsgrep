use regex::{Regex, RegexSet};
use reqwest;
use scraper::{error::SelectorErrorKind, Html, Selector};

// this is to get rid of warnings when compiling
#[allow(dead_code)]
#[derive(Clone)]
pub enum SearchTerms {
    Words(Regex),       // singular regex expression
    Patterns(RegexSet), // multiple regex expressions
    None,               // nothing
}

#[derive(Clone)]
pub struct Article {
    pub url: String,        // the URL of the website
    pub terms: SearchTerms, // for filtering out articles
    pub selector: String,   // for scraping articles from the HTTP request
}

pub trait Scrapeable {
    /// these are the traits that should be implemented

    fn find_articles(&self, body: &String) -> Result<Vec<Article>, SelectorErrorKind>;

    fn summarize_article(&self) -> String;

    //fn transmute_article_links(&self, articles: &mut Vec<Article>);

    fn filter_by_url(&self, articles: &Vec<Article>) -> Option<Vec<Article>>;

    fn print_articles(&self, articles: &Vec<Article>) {
        println!("Found articles from {}...", self.summarize_article());
        articles
            .iter()
            .for_each(|article| println!("article: {:?}", article.url));
    }
}

#[allow(dead_code)]
impl Article {
    /// this impl is for base basic object management
    /// e.g., constructors, getters, setters, etc.

    /// Creates a Article struct from a URL and some search terms
    pub fn new(url: String, terms: SearchTerms, selector: String) -> Article {
        Article {
            url,
            terms,
            selector,
        }
    }

    /// Creates an empty Article struct
    pub fn empty() -> Article {
        Self::new(String::new(), SearchTerms::None, String::new())
    }

    /// Creates a Article struct with a URL
    pub fn with_url(url: String) -> Article {
        Self::new(url, SearchTerms::None, String::from("a"))
    }

    /// Creates a Article struct with a URL and selector
    pub fn with_url_and_sel(url: String, selector: String) -> Article {
        Self::new(url, SearchTerms::None, selector)
    }

    /// getters/setters

    pub fn set_url(&mut self, new_url: String) {
        self.url = new_url;
    }

    pub fn set_terms(&mut self, new_terms: SearchTerms) {
        self.terms = new_terms;
    }

    pub fn set_selector(&mut self, new_selector: String) {
        self.selector = new_selector;
    }

    /// this function makes an HTTP GET request to the URL
    pub async fn get(&self) -> Result<String, reqwest::Error> {
        // makes a GET request to the URL and returns the body of the
        // response, error otherwise
        reqwest::get(&self.url).await?.text().await
    }
}

impl Scrapeable for Article {
    /// Nicely prints the name of the article
    fn summarize_article(&self) -> String {
        format!("{:?}", self.url)
    }

    // make this function return Option<Vec<Article>> or a Result<Option, E>
    fn filter_by_url(&self, articles: &Vec<Article>) -> Option<Vec<Article>> {
        // no articles therefore None
        if articles.len() == 0 {
            return None;
        };

        // make new Vec of articles
        let mut filtered: Vec<Article> = Vec::new();

        // applying our search terms to each article's URL
        match &self.terms {
            SearchTerms::Words(words) => {
                // filtering articles
                articles.iter().for_each(|article| {
                    if words.is_match(&article.url) {
                        filtered.push(article.clone());
                    }
                });
            }
            SearchTerms::Patterns(patterns) => {
                // filtering articles
                articles.iter().for_each(|article| {
                    if patterns.is_match(&article.url) {
                        filtered.push(article.clone());
                    }
                });
            }
            SearchTerms::None => return None,
        };

        // returning None if we didn't find any articles
        // conforming to the SearchTerms
        if filtered.len() != 0 {
            Some(filtered)
        } else {
            None
        }
    }

    /// Uses HTML scraping in order to extract each <a> tag
    fn find_articles(&self, body: &String) -> Result<Vec<Article>, SelectorErrorKind> {
        // creating storage for all the <a> tags found
        let mut articles = Vec::new();

        // creating a selector to target articles
        let sel = match Selector::parse(&self.selector) {
            Ok(s) => s,
            Err(error) => return Err(error),
        };

        // casting the body into a document
        let document = Html::parse_document(&body);

        // parsing the HTML body
        for element in document.select(&sel) {
            match element.value().attr("href") {
                Some(attr) => articles.push(Self::with_url(attr.to_string())),
                None => eprintln!("Couldn't extract href from this element"),
            }
        }

        // returning the articles we found
        Ok(articles)
    }
}
