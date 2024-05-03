use reqwest::{header::HeaderValue, Client, IntoUrl};
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use time::{formatting::Formattable, OffsetDateTime};

use crate::item::Item;

const FLARE_SOLVER: &'static str = "http://localhost:8191/v1";

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
            name_selector: r#"a.product--title > br"#,
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
        let text = if self.bypass_cloudflare {
            Self::bypass_search(client, url).await?
        } else {
            Self::direct_search(client, url).await?
        };
        let document = Html::parse_document(&text);

        let inner_html_f = |e: ElementRef| html_escape::decode_html_entities(&e.inner_html()).trim().to_owned();

        let price = Self::select(&document, &self.price_selector, inner_html_f)?;
        let image_link = self.img_link(&document)?;
        let name = Self::select(&document, &self.name_selector, inner_html_f)?;
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

    fn product_link(&self, document: &Html) -> anyhow::Result<String> {
        let f = move |e: ElementRef| {
            let res = match *self {
                Self::BIKE_DISCOUNT => e.attr("href").map(|s| s.to_owned()),
                _ => e.attr("href").map(|s| [self.top_level_domain, s].concat()),
            };
            res.unwrap_or("not_found".to_owned())
        };
        Self::select(&document, &self.link_selector, f)
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
        Self::select(document, &self.image_selector, f)
    }

    fn select<S: Fn(ElementRef) -> String>(
        document: &Html,
        selector: &str,
        f: S
    ) -> anyhow::Result<String> {
        let selector = Selector::parse(selector)
            // no send for errors
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        document
            .select(&selector)
            .next()
            .map(|er| f(er))
            .ok_or(anyhow::anyhow!("selector not found"))
    }

    fn search_url(&self, search_term: &str) -> String {
        let encoded_search_term = urlencoding::encode(search_term).into_owned();
        [self.top_level_domain, self.search_prefix, &encoded_search_term].concat()
    }

    async fn direct_search<T>(
        client: &Client,
        url: T,
    ) -> anyhow::Result<String> where T: IntoUrl {
        let resp = client.get(url).send().await?;
        resp.text().await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    async fn bypass_search<T>(
        client: &Client,
        url: T,
    ) -> anyhow::Result<String> where T: IntoUrl {
        let req_body = Self::bypass_req_body(url.as_str());
        let resp = client
            .post(FLARE_SOLVER)
            .body(req_body)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .send()
            .await?;
        let text = resp.text().await?;
        let json: Value = serde_json::from_str(&text)?;
        json["solution"]["response"]
            .as_str()
            .map(|s| s.to_owned())
            .ok_or(anyhow::anyhow!("solution.response could not be found in json"))
    }

    fn bypass_req_body(url: &str) -> String {
        serde_json::json!({"cmd": "request.get", "url": url, "maxTimeout": 60000}).to_string()
    }
}