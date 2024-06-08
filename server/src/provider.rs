use std::{hash::RandomState, sync::Arc};

use domain::item::Item;
use moka::future::Cache;
use reqwest::{Client, IntoUrl};
use scraper::{ElementRef, Html};
use strsim::jaro_winkler;
use time::{formatting::Formattable, OffsetDateTime};

use crate::{selecting::{html_select::HtmlSelect, selector::Selector}, solving::{solution::CachedSolution, solver::Solver}};

const SIM_THRESHOLD: f64 = 0.5;


#[derive(Eq, PartialEq)]
pub struct Provider {
    name: &'static str,
    top_level_domain: &'static str,
    search_prefix: &'static str,
    name_selector: Selector,
    link_selector: Selector,
    price_selector: Selector,
    image_selector: Selector,
    logo_link: &'static str,
    bypass_cloudflare: bool,
}

impl Provider {
    pub const ALL: [Provider; 6] = [
        Self::ALLTRICKS,
        Self::BIKE_COMPONENTS,
        Self::BIKE_DISCOUNT,
        Self::BIKE_INN,
        Self::LORDGUN,
        Self::STARBIKE,
    ];

    pub const ALLTRICKS: Provider = 
        Provider {
            name: "Alltricks",
            top_level_domain: "https://www.alltricks.fr",
            search_prefix: "/Acheter/",
            name_selector: Selector::direct(r#"a.alltricks-Product-description"#),
            link_selector: Selector::direct(r#"a.alltricks-Product-description"#),
            price_selector: Selector::direct(r#"span.alltricks-Product-price.alltricks-Product-actualPrice > span"#),
            image_selector: Selector::direct(r#"span.alltricks-Product-picture > img"#),
            logo_link: "https://www.alltricks.fr/fstrz/r/s/www.alltricks.fr/images/2022_ALLTRICKS_QUADRI_ORIGINAL_BLANC.svg",
            bypass_cloudflare: false,
        };

    pub const BIKE_DISCOUNT: Provider = 
        Provider {
            name: "Bike-Discount",
            top_level_domain: "https://www.bike-discount.de",
            search_prefix: "/en/search?sSearch=",
            name_selector: Selector::direct(r#"a.product--title"#),
            link_selector: Selector::direct(r#"a.product--title"#),
            price_selector: Selector::direct(r#"span.price--default.is--nowrap.is--discount"#),
            image_selector: Selector::direct(r#"span.image--media > img"#),
            logo_link: "https://cdn.starbike.com/logo.svg",
            bypass_cloudflare: true,
        };

    pub const STARBIKE: Provider =
        Provider {
            name: "Starbike",
            top_level_domain: "https://www.starbike.com",
            search_prefix: "/en/search/?q=",
            name_selector: Selector::direct(r#"a.pb-link"#),
            link_selector: Selector::direct(r#"a.pb-link"#),
            price_selector: Selector::direct(r#"span.productbox-price"#),
            image_selector: Selector::direct(r#"li.uk-margin-remove-top.uk-position-relative.uk-display-block div.uk-text-center.uk-position-relative > img.pb-link-trigger.product-box.productbox-image"#),
            logo_link: "https://cdn.starbike.com/logo.svg",
            bypass_cloudflare: false,
        };

    pub const LORDGUN: Provider =
        Provider {
            name: "Lordgun",
            top_level_domain: "https://www.lordgunbicycles.fr",
            search_prefix: "/recherche?s=",
            name_selector: Selector::direct(r#"a.article__link"#),
            link_selector: Selector::direct(r#"a.article__link"#),
            price_selector: Selector::direct(r#"span.product__price > span > span:nth-child(2) > strong"#),
            image_selector: Selector::direct(r#"figure.article__figure > a.link > img"#),
            logo_link: "https://sync.lordgunbicycles.com:4433/img/logo.lordgun.svg",
            bypass_cloudflare: false,
        };

    pub const BIKE_COMPONENTS: Provider =
        Provider {
            name: "Bike-Components",
            top_level_domain: "https://www.bike-components.de",
            search_prefix: "/en/s/?keywords=",
            // has html comments
            name_selector: Selector::direct(r#"h3.headline.site-headline-s-lowercase"#),
            link_selector: Selector::direct(r#"a.product-item.js-product-item"#),
            price_selector: Selector::direct(r#"div.prices > div.price.site-price"#),
            image_selector: Selector::relative(r#"div.site-product-image.relative > picture > img.object-bottom"#),
            // not sure it works
            logo_link: "https://www.bike-components.de/assets/build/9c5f83272131c0d32378-spritemap.svg",
            bypass_cloudflare: false,
        };

    pub const BIKE_INN: Provider =
        Provider {
            name: "bike INN",
            top_level_domain: "https://www.tradeinn.com",
            search_prefix: "/bikeinn/fr?products_search[query]=",
            name_selector: Selector::direct(r#"h3 > p.txt-base.js-nombre_producto_listado"#),
            link_selector: Selector::relative(r#"a.js-href_list_products"#),
            price_selector: Selector::direct(r#"p.txt-important.txt-precio__listado.js-precio_producto"#),
            image_selector: Selector::direct(r#"div.listado-foto__mainfoto > img.js-image_list_product"#),
            logo_link: "https://cache.tradeinn.com/images/web/bikeinn.svg",
            bypass_cloudflare: false,
        };

    pub async fn crawl<F>(
        &self,
        client: &Client,
        cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
        solver: Solver,
        search_term: &str,
        date_format: F,
    ) -> anyhow::Result<Option<Item>> where F: Formattable + Sized {
        let search_url = self.search_url(search_term);

        let text = if self.bypass_cloudflare {
            solver.solve(client, &cookie_cache, self.name, search_url).await?
        } else {
            Self::direct_search(client, search_url).await?
        };

        let document = Html::parse_document(&text);

        let inner_html_f = |e: ElementRef| html_escape::decode_html_entities(&e.inner_html())
            .trim()
            .to_owned();

        // todo: refactor
        let price = self.price(&document, inner_html_f)?;
        let image_link = self.img_link(&document)?;
        let name = self.name(&document, inner_html_f)?;
        let product_link = self.product_link(&document)?;

        let dt = OffsetDateTime::now_utc().format(&date_format)?;

        let sim = jaro_winkler(&name, search_term);

        if sim > SIM_THRESHOLD {
            Ok(Some(Item {
                name,
                provider: self.name.to_owned(),
                price,
                image_link,
                product_link,
                logo_link: self.logo_link.to_owned(),
                time: dt,
            }))
        } else {
            println!("discarding {} with similarity {}", self.name, sim);
            Ok(None)
        }
    }

    async fn direct_search<T>(client: &Client, url: T) -> anyhow::Result<String> where T: IntoUrl {
        let resp = client.get(url).send().await?;
        resp.text().await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    // todo: extract link parsing
    fn name(&self, document: &Html, inner_html_f: fn(ElementRef) -> String) -> anyhow::Result<String> {
        let f = move |e: ElementRef| inner_html_f(e)
            .split("<br>")
            .last()
            // bike-components
            .map(|s| s.replace("<!--[-->", "").replace("<!--]-->", ""))
            .unwrap_or("not_found".to_string());
        document.html_select(&self.name_selector.s, f)
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
        document.html_select(&self.price_selector.s, f)
    }

    fn product_link(&self, document: &Html) -> anyhow::Result<String> {
        let f = move |e: ElementRef| {
            let res = match *self {
                Self::BIKE_DISCOUNT => e.attr("href").map(|s| s.to_owned()),
                _ => e.attr("href").map(|s| [self.top_level_domain, s].concat()),
            };
            res.unwrap_or("not_found".to_owned())
        };
        // todo: refactor
        let Selector { s, relative } = self.link_selector;
        let selected = document.html_select(s, f)?;
        if relative {
            Ok(self.top_level_domain.to_owned() + &selected)
        } else {
            Ok(selected)
        }
    }

    fn img_link(&self, document: &Html) -> anyhow::Result<String> {
        let f = move |e: ElementRef| {
            let res = match *self {
                Self::BIKE_DISCOUNT => e.attr("srcset").and_then(|s| s.split(",").next()).map(|s| s.to_owned()),
                Self::STARBIKE => e.attr("lazyload").map(|s| s.replace("%W%", "200").replace("%H%", "200")),
                Self::LORDGUN => e.attr("data-src").map(|s| s.to_owned()),
                _ => e.attr("src").map(|s| s.to_owned()),
            };
            res.unwrap_or("not found".to_owned())
        };
        // todo: refactor
        let Selector { s, relative } = self.image_selector;
        let selected = document.html_select(s, f)?;
        if relative {
            Ok(self.top_level_domain.to_owned() + &selected)
        } else {
            Ok(selected)
        }
    }

    fn search_url(&self, search_term: &str) -> String {
        let encoded_search_term = urlencoding::encode(search_term).into_owned();
        [self.top_level_domain, self.search_prefix, &encoded_search_term].concat()
    }
}