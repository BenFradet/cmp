use reqwest::{header::HeaderValue, Client, IntoUrl};
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use time::{format_description, OffsetDateTime};

use crate::item::Item;

const FLARE_SOLVER: &'static str = "http://localhost:8191/v1";
const DATE_FORMAT_STR: &'static str = "[year]-[month]-[day]-[hour]:[minute]:[second]";

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
            top_level_domain: "https://www.bike-discount.de/en",
            search_prefix: "/search?sSearch=",
            name_selector: r#"a.alltricks-Product-description"#,
            link_selector: r#"a.alltricks-Product-description"#,
            price_selector: r#"span.price--default.is--nowrap.is--discount"#,
            image_selector: r#"span.alltricks-Product-picture > img"#,
            logo_link: "",
            bypass_cloudflare: true,
        };

    pub const STARBIKE: Provider =
        Provider {
            name: "starbike",
            top_level_domain: "https://www.starbike.com/en",
            search_prefix: "/search/?q=",
            name_selector: r#"a.alltricks-Product-description"#,
            link_selector: r#"a.alltricks-Product-description"#,
            price_selector: r#"span.productbox-price"#,
            image_selector: r#"span.alltricks-Product-picture > img"#,
            logo_link: "",
            bypass_cloudflare: false,
        };

    pub async fn crawl(&self, client: &Client, search_term: &str) -> anyhow::Result<String> {
        let search_url = self.search_url(search_term);
        self.search(client, search_url).await
    }

    async fn search<T>(
        &self,
        client: &Client,
        url: T,
    ) -> anyhow::Result<String> where T: IntoUrl {
        // todo cache and reuse the cookie which is sent back
        let text = if self.bypass_cloudflare {
            Self::bypass_search(client, url).await?
        } else {
            Self::direct_search(client, url).await?
        };
        let document = Html::parse_document(&text);

        let inner_html_f = |e: ElementRef| html_escape::decode_html_entities(&e.inner_html()).trim().to_owned();
        let src_f = |e: ElementRef| e.attr("src").unwrap_or("not found").to_owned();
        let href_f = |e: ElementRef| e.attr("href").unwrap_or("not found").to_owned();

        let price = Self::select(&document, &self.price_selector, inner_html_f)?;
        let image_link = Self::select(&document, &self.image_selector, src_f)?;
        let name = Self::select(&document, &self.name_selector, inner_html_f)?;
        let link = Self::select(&document, &self.link_selector, href_f)?;

        let fmt = format_description::parse(DATE_FORMAT_STR)?;
        let dt = OffsetDateTime::now_utc().format(&fmt)?;
        Item {
            name: name,
            provider: self.name,
            price: price,
            image_link: image_link,
            link: link,
            logo_link: self.logo_link,
            time: dt,
        };

        let selector = Selector::parse(&self.price_selector)
            // no send for errors
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        document
            .select(&selector)
            .next()
            .map(|er| html_escape::decode_html_entities(&er.inner_html()).trim().to_string())
            .ok_or(anyhow::anyhow!("selector not found"))
    }

    fn select(
        document: &Html,
        selector: &str,
        f: fn(ElementRef) -> String,
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