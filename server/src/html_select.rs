use scraper::{ElementRef, Html, Selector};

pub trait HtmlSelect {
    fn html_select<A, S: Fn(ElementRef) -> A>(
        &self,
        selector: &str,
        f: S
    ) -> anyhow::Result<A>;
}

impl HtmlSelect for Html {
    fn html_select<A, S: Fn(ElementRef) -> A>(
            &self,
            selector: &str,
            f: S
        ) -> anyhow::Result<A> {
        let selector = Selector::parse(selector)
            // no send for errors
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        self
            .select(&selector)
            .next()
            .map(|er| f(er))
            .ok_or(anyhow::anyhow!("selector not found"))
        
    }
}