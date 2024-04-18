use std::error::Error;

use reqwest::IntoUrl;
use scraper::{Html, Selector};

use crate::er::Er;

pub trait Price {
    fn price_selector(&self) -> &str;

    async fn price<T>(&self, url: T) -> Result<String, Box<dyn Error>> where T: IntoUrl {
        let resp = reqwest::get(url).await?;
        let text = resp.text().await?;
        let document = Html::parse_document(&text);
        let selector = Selector::parse(self.price_selector())
            .map_err(|e| e.to_string())?;
        document
            .select(&selector)
            .next()
            .map(|er| er.inner_html())
            .ok_or(Box::new(Er::new("selector not found")))
    }
}