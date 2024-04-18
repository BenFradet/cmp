use std::error::Error;

use crate::{price::Price, search::Search, urls::Urls};

pub trait Crawl {
    async fn crawl(&self, search_term: &str) -> Result<String, Box<dyn Error>>;
}

impl<A> Crawl for A where A: Urls + Search + Price {
    async fn crawl(&self, search_term: &str) -> Result<String, Box<dyn Error>> {
        let search_url = self.search_url(search_term);
        let fragment_url = self.search(search_url).await?;
        let item_url = [self.top_level_domain(), &fragment_url].concat();
        self.price(&item_url).await
    }
}