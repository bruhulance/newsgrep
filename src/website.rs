use regex::RegexSet;
use reqwest;

pub enum SearchTerms {
    Words(Vec<String>),
    Patterns(RegexSet),
    None
}

pub struct Website {
    pub url: String,
    pub terms_found: i32,
    pub terms: SearchTerms
}

impl Website {
    pub fn new() -> Website {
        Website {
            url: String::new(),
            terms_found: 0,
            terms: SearchTerms::None
        }
    }

    pub async fn make_get(&self) -> Result<(), reqwest::Error> {
        // getting a response from the website
        let res = reqwest::get(&self.url).await?;
    
        println!("Response: {:?} {}", res.version(), res.status());
        println!("Headers: {:#?}\n", res.headers());
    
        let body = res.text().await?;
    
        println!("{}", body);
    
        Ok(())
    }
}

