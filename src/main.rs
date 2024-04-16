use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.alltricks.fr/F-11944-selles/P-1937973-selle_italia_slr_boost_endurance_superflow_noir";
    let resp = reqwest::get(url).await?;
    println!("{}", resp.url().to_string());

    let text = resp.text().await?;

    //let mut urls: HashSet<String> = HashSet::new();
    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"div#content-wrap > div#content > div#content-product > div#product-header > div#product-header-order > div#product-header-order-form > form#form_current_product > div.blue-box > div.product-header-order-price > div.prices > div.reduction > p.price > span"#).unwrap();
    for price in document.select(&selector) {
        println!("{:?}", price.inner_html());
    }

    Ok(())
}
