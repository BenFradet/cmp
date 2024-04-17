use std::error::Error;

use crate::{er::Er, price::Price};
use scraper::{Html, Selector};

pub struct Alltricks<'a> {
    price_selector: &'a str
}

impl Default for Alltricks<'_> {
    fn default() -> Self {
        Alltricks {
            price_selector: r#"div#content-wrap > div#content > div#content-product > div#product-header > div#product-header-order > div#product-header-order-form > form#form_current_product > div.blue-box > div.product-header-order-price > div.prices > div.reduction > p.price > span"#,
        }
    }
}

impl Price for Alltricks<'_> {
    async fn price<T>(&self, url: T) -> Result<String, Box<dyn Error>> where T: reqwest::IntoUrl {
        let resp = reqwest::get(url).await?;
        let text = resp.text().await?;
        let document = Html::parse_document(&text);
        let selector = Selector::parse(self.price_selector).unwrap();
        document
            .select(&selector)
            .next()
            .map(|s| s.inner_html())
            .ok_or(Box::new(Er::new("selector not found")))
    }
}