use std::error::Error;

use reqwest::IntoUrl;
use scraper::{Html, Selector};

use crate::er::Er;

pub trait Search {
    fn search_selector(&self) -> &str;

    async fn search<T>(&self, url: T) -> Result<String, Box<dyn Error>> where T: IntoUrl {
        let resp = reqwest::get(url).await?;
        let text = resp.text().await?;
        let document = Html::parse_document(&text);
        let selector = Selector::parse(self.search_selector())
            .map_err(|e| e.to_string())?;
        document
            .select(&selector)
            .next()
            .and_then(|er| {
                println!("{:?}", er.inner_html());
                er.attr("href").map(|l| l.to_owned())
            })
            .ok_or(Box::new(Er::new("selector or attr href not found")))
    }
}