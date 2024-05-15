use domain::item::Item;
use reqwest::{Client, IntoUrl};
use scraper::{ElementRef, Html};
use time::{formatting::Formattable, OffsetDateTime};

use crate::{html_select::HtmlSelect, services::search::Search};

#[derive(Eq, PartialEq)]
pub struct Provider {
    name: &'static str,
    top_level_domain: &'static str,
    search_prefix: &'static str,
    name_selector: &'static str,
    link_selector: &'static str,
    price_selector: &'static str,
    image_selector: &'static str,
    logo_link: &'static str,
    bypass_cloudflare: bool,
}

impl Provider {
    pub const ALLTRICKS: Provider = 
        Provider {
            name: "Alltricks",
            top_level_domain: "https://www.alltricks.fr",
            search_prefix: "/Acheter/",
            name_selector: r#"a.alltricks-Product-description"#,
            link_selector: r#"a.alltricks-Product-description"#,
            price_selector: r#"span.alltricks-Product-price.alltricks-Product-actualPrice > span"#,
            image_selector: r#"span.alltricks-Product-picture > img"#,
            logo_link: "https://www.alltricks.fr/fstrz/r/s/www.alltricks.fr/images/2022_ALLTRICKS_QUADRI_ORIGINAL_BLANC.svg",
            bypass_cloudflare: false,
        };

    pub const BIKE_DISCOUNT: Provider = 
        Provider {
            name: "Bike-Discount",
            top_level_domain: "https://www.bike-discount.de",
            search_prefix: "/en/search?sSearch=",
            name_selector: r#"a.product--title"#,
            link_selector: r#"a.product--title"#,
            price_selector: r#"span.price--default.is--nowrap.is--discount"#,
            image_selector: r#"span.image--media > img"#,
            logo_link: "https://cdn.starbike.com/logo.svg",
            bypass_cloudflare: true,
        };

    pub const STARBIKE: Provider =
        Provider {
            name: "starbike",
            top_level_domain: "https://www.starbike.com",
            search_prefix: "/en/search/?q=",
            name_selector: r#"a.pb-link"#,
            link_selector: r#"a.pb-link"#,
            price_selector: r#"span.productbox-price"#,
            image_selector: r#"li.uk-margin-remove-top.uk-position-relative.uk-display-block div.uk-text-center.uk-position-relative > img.pb-link-trigger.product-box.productbox-image"#,
            logo_link: "https://cdn.starbike.com/logo.svg",
            bypass_cloudflare: false,
        };

    pub async fn crawl<F>(
        &self,
        client: &Client,
        search_term: &str,
        date_format: F,
    ) -> anyhow::Result<Item> where F: Formattable + Sized {
        let search_url = self.search_url(search_term);
        self.search(client, search_url, date_format).await
    }

    async fn search<T, F>(
        &self,
        client: &Client,
        url: T,
        date_format: F,
    ) -> anyhow::Result<Item> where T: IntoUrl, F: Formattable + Sized {
        // todo cache and reuse the cookie which is sent back
        let text = client.search(url, self.bypass_cloudflare).await?;
        let document = Html::parse_document(&text);

        let inner_html_f = |e: ElementRef| html_escape::decode_html_entities(&e.inner_html())
            .trim()
            .to_owned();

        let price = self.price(&document, inner_html_f)?;
        let image_link = self.img_link(&document)?;
        let name = self.name(&document, inner_html_f)?;
        let product_link = self.product_link(&document)?;

        let dt = OffsetDateTime::now_utc().format(&date_format)?;
        Ok(Item {
            name,
            provider: self.name.to_owned(),
            price,
            image_link,
            product_link,
            logo_link: self.logo_link.to_owned(),
            time: dt,
        })
    }

    fn name(&self, document: &Html, inner_html_f: fn(ElementRef) -> String) -> anyhow::Result<String> {
        let f = move |e: ElementRef| inner_html_f(e)
            .split("<br>")
            .last()
            .unwrap_or("not_found")
            .to_owned();
        document.html_select(&self.name_selector, f)
    }

    fn price(&self, document: &Html, inner_html_f: fn(ElementRef) -> String) -> anyhow::Result<Option<f64>> {
        let f = move |e: ElementRef| {
            inner_html_f(e)
                .replace("€", "")
                // parser is mega finicky
                .replace(",", ".")
                .trim()
                .parse::<f64>()
                .ok()
        };
        document.html_select(&self.price_selector, f)
    }

    fn product_link(&self, document: &Html) -> anyhow::Result<String> {
        let f = move |e: ElementRef| {
            let res = match *self {
                Self::BIKE_DISCOUNT => e.attr("href").map(|s| s.to_owned()),
                _ => e.attr("href").map(|s| [self.top_level_domain, s].concat()),
            };
            res.unwrap_or("not_found".to_owned())
        };
        document.html_select(&self.link_selector, f)
    }

    fn img_link(&self, document: &Html) -> anyhow::Result<String> {
        let f = move |e: ElementRef| {
            let res = match *self {
                Self::ALLTRICKS => e.attr("src").map(|s| s.to_owned()),
                Self::BIKE_DISCOUNT => e.attr("srcset").and_then(|s| s.split(",").next()).map(|s| s.to_owned()),
                Self::STARBIKE => e.attr("lazyload").map(|s| s.replace("%W%", "200").replace("%H%", "200")),
                _ => None,
            };
            res.unwrap_or("not found".to_owned())
        };
        document.html_select(&self.image_selector, f)
    }

    fn search_url(&self, search_term: &str) -> String {
        let encoded_search_term = urlencoding::encode(search_term).into_owned();
        [self.top_level_domain, self.search_prefix, &encoded_search_term].concat()
    }
}