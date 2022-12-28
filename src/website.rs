use scraper::{Html, Selector, error::SelectorErrorKind};
use regex::RegexSet;
use reqwest;

// this is to get rid of warnings when compiling
#[allow(dead_code)]
pub enum SearchTerms {
    Words(Vec<String>),
    Patterns(RegexSet),
    None
}

pub struct Website {
    pub url: String,        // the URL of the website
    pub terms: SearchTerms, // for filtering out articles
    pub selector: String    // for scraping articles from the HTTP request 
}

// this is to get rid of warnings when compiling
#[allow(dead_code)]
impl Website {
    /// constructors
   
    /// Creates a Website struct from a URL and some search terms
    pub fn new(url: String, terms: SearchTerms, selector: String) -> Website {
        Website {
            url,
            terms,
            selector
        }
    }

    /// Creates an empty Website struct
    pub fn empty() -> Website {
        Self::new(String::new(), SearchTerms::None, String::new())
    }

    /// Creates a Website struct with a URL
    pub fn with_url(url: String) -> Website {
        Self::new(url, SearchTerms::None, String::from("a"))
    }

    /// Creates a Website struct with a URL and selector
    pub fn with_url_and_sel(url: String, selector: String) -> Website {
        Self::new(url, SearchTerms::None, selector)
    }

    /// getters/setters

    pub fn set_terms(&mut self, new_terms: SearchTerms) {
        self.terms = new_terms;
    }

    pub fn set_url(&mut self, new_url: String) {
        self.url = new_url;
    }

    /// methods

    pub fn find_articles(&self, body: &String) -> Result<Vec<Website>, SelectorErrorKind> {
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
                None => eprintln!("Couldn't extract href from this element")
            }
        }

        // returning the articles we found
        Ok(articles)
    }

    /// this function makes an HTTP GET request to the URL
    pub async fn make_get(&self) -> Result<String, reqwest::Error> {
        // makes a GET request to the URL and returns the body of the
        // response, error otherwise
        reqwest::get(&self.url).await?.text().await
    }
}

