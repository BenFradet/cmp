pub trait Urls {
    fn top_level_domain(&self) -> &str;
    fn search_prefix(&self) -> &str;
    fn search_url(&self, search_term: &str) -> String {
        let encoded_search_term = urlencoding::encode(search_term).into_owned();
        [self.top_level_domain(), self.search_prefix(), &encoded_search_term].concat()
    }
}