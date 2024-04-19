use reqwest::{header::HeaderValue, Client, IntoUrl};
use scraper::{Html, Selector};
use serde_json::Value;

pub struct Provider<'a> {
    top_level_domain: &'a str,
    search_prefix: &'a str,
    price_selector: &'a str,
    bypass_cloudflare: bool,
}

impl<'a> Provider<'a> {
    pub const ALLTRICKS: Provider<'a> = 
        Provider {
            top_level_domain: "https://www.alltricks.fr",
            search_prefix: "/Acheter/",
            price_selector: r#"span.alltricks-Product-price.alltricks-Product-actualPrice > span"#,
            bypass_cloudflare: false,
        };

    pub const BIKE_DISCOUNT: Provider<'a> = 
        Provider {
            top_level_domain: "https://www.bike-discount.de/en",
            search_prefix: "/search?sSearch=",
            price_selector: r#"span.price--default.is--nowrap.is--discount"#,
            bypass_cloudflare: true,
        };

    pub const STARBIKE: Provider<'a> =
        Provider {
            top_level_domain: "https://www.starbike.com/en",
            search_prefix: "/search/?q=",
            price_selector: r#"span.productbox-price"#,
            bypass_cloudflare: false,
        };

    pub const FLARE_SOLVER: &'static str = "http://localhost:8191/v1";

    pub async fn crawl(&self, client: &Client, search_term: &str) -> anyhow::Result<String> {
        let search_url = self.search_url(search_term);
        self.search(client, search_url).await
    }

    async fn search<T>(
        &self,
        client: &Client,
        url: T,
    ) -> anyhow::Result<String> where T: IntoUrl {
        let text = if self.bypass_cloudflare {
            Self::bypass_search(client, url).await?
        } else {
            Self::direct_search(client, url).await?
        };
        let document = Html::parse_document(&text);
        let selector = Selector::parse(&self.price_selector)
            // no send for errors
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        document
            .select(&selector)
            .next()
            .map(|er| {
                println!("{}", er.inner_html());
                html_escape::decode_html_entities(&er.inner_html()).trim().to_string()
            })
            .ok_or(anyhow::anyhow!("selector not found"))
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
            .post(Self::FLARE_SOLVER)
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

    fn search_url(&self, search_term: &str) -> String {
        let encoded_search_term = urlencoding::encode(search_term).into_owned();
        [self.top_level_domain, self.search_prefix, &encoded_search_term].concat()
    }
}