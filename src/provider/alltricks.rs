use crate::{price::Price, search::Search, urls::Urls};

pub struct Alltricks<'a> {
    top_level_domain: &'a str,
    search_prefix: &'a str,
    price_selector: &'a str,
    search_selector: &'a str,
}

impl Default for Alltricks<'_> {
    fn default() -> Self {
        Alltricks {
            top_level_domain: "https://www.alltricks.fr",
            search_prefix: "/Acheter/",
            price_selector: r#"div#content-wrap > div#content > div#content-product > div#product-header > div#product-header-order > div#product-header-order-form > form#form_current_product > div.blue-box > div.product-header-order-price > div.prices > div.reduction > p.price > span"#,
            search_selector: r#"div#content-wrap > div#content > div.alltricks-ProductListing.row > div.alltricks-ProductListing__displayProducts > div.alltricks-ProductListing__content > div#alltricks-Pager > div.alltricks-Pager__item > div.alltricks-Product--3columns > div.alltricks-Product.alltricks-Product--grid > div.alltricks-Product-link-wrapper > a.alltricks-Product-description"#,
        }
    }
}

impl Price for Alltricks<'_> {
    fn price_selector(&self) -> &str {
        self.price_selector
    }
}

impl Search for Alltricks<'_> {
    fn search_selector(&self) -> &str {
        self.search_selector
    }
}

impl Urls for Alltricks<'_> {
    fn top_level_domain(&self) -> &str {
        self.top_level_domain
    }

    fn search_prefix(&self) -> &str {
        self.search_prefix
    }
}