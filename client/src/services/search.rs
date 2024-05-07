use domain::{item::Item, response::Response};
use gloo::net::http::Request;

const INITIAL_SEARCH_TERM: &'static str = "Selle italia slr boost endurance";

pub async fn search(search_term: Option<&str>) -> anyhow::Result<Vec<Item>> {
    let q = search_term.unwrap_or(INITIAL_SEARCH_TERM);
    let encoded_q = urlencoding::encode(&q);
    let url = format!("/api/v1/search?q={encoded_q}");

    // investigate cache
    Ok(Request::get(&url)
        .send()
        .await?
        .json::<Response>()
        .await
        .map(|r| r.items)?
    )
}