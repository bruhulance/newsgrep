use async_trait::async_trait;
use regex::RegexSet;
use reqwest;

// this is to get rid of warnings when compiling
#[allow(dead_code)]
pub enum SearchTerms {
    Words(Vec<String>),
    Patterns(RegexSet),
    None,
}

pub struct Article {
    pub url: String,        // the URL of the website
    pub terms: SearchTerms, // for filtering out articles
    pub selector: String,   // for scraping articles from the HTTP request
}

#[async_trait]
pub trait Website {
    /// this function makes an HTTP GET request to the URL
    async fn get(&self) -> Result<String, reqwest::Error>;
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
}

#[async_trait]
impl Website for Article {
    /// this function makes an HTTP GET request to the URL
    async fn get(&self) -> Result<String, reqwest::Error> {
        // makes a GET request to the URL and returns the body of the
        // response, error otherwise
        reqwest::get(&self.url).await?.text().await
    }
}

//pub trait Scrapable {
//    /// these are the traits that should be implemented
//
//    fn find_articles(&self, body: &String) -> Result<Vec<dyn Website>, SelectorErrorKind>;
//
//    fn transmute_article_links(&self, articles: &mut Vec<dyn Website>) -> Result<Vec<dyn Website>, ()>;
//
//    fn print_articles(&self, articles: &Vec<dyn Website>);
//}
//
