
    fn find_articles(&self, body: &String) -> Result<Vec<Website>, SelectorErrorKind> {
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


pub trait Scrapable {
    /// these are the traits that should be implemented

    fn find_articles(&self, body: &String) -> Result<Vec<dyn Website>, SelectorErrorKind>;

    fn transmute_article_links(&self, articles: &mut Vec<dyn Website>) -> Result<Vec<dyn Website>, ()>;

    fn print_articles(&self, articles: &Vec<dyn Website>);
}

