use std::{convert::Infallible, hash::RandomState, sync::Arc};

use domain::{item::Item, response::Response};
use futures::stream::{self, StreamExt};
use moka::future::Cache;
use reqwest::Client;
use warp::reply::Reply;

use crate::{provider::Provider, solving::{solution::CachedSolution, solver::Solver}};

const PARALLEL_REQUESTS: usize = 6;
const DATE_FORMAT_STR: &'static str = "[year]-[month]-[day]-[hour]:[minute]:[second]";

pub async fn search(
    search_term: String,
    client: Client,
    items_cache: Cache<String, Arc<Vec<Item>>, RandomState>,
    cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
    solver: Solver,
) -> Result<impl Reply, Infallible> {
    println!("received: {search_term}");
    let res = fetch(&search_term, client, items_cache, cookie_cache, solver).await;
    println!("results: {:?}", res);
    Ok(warp::reply::json(&Response { items: res.to_vec() }))
}

async fn fetch(
    search_term: &str,
    client: Client,
    items_cache: Cache<String, Arc<Vec<Item>>, RandomState>,
    cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
    solver: Solver,
) -> Arc<Vec<Item>> {
    let providers_len = Provider::ALL.len();

    let date_fmt = time::format_description::parse(&DATE_FORMAT_STR).unwrap();

    let sanitized_search_term = sanitize_search_term(search_term);

    if let Some(items) = items_cache.get(&sanitized_search_term).await {
        items
    } else {
        let results = stream::iter(Provider::ALL)
            .map(|p| {
                let client = client.clone();
                let cookie_cache = cookie_cache.clone();
                let solver = solver.clone();
                // corouting moves
                let s = sanitized_search_term.to_owned();
                let fmt = date_fmt.clone();
                tokio::spawn(async move {
                    p.crawl(&client, cookie_cache, solver, &s, fmt).await
                })
            })
            .buffer_unordered(PARALLEL_REQUESTS)
            .take(providers_len)
            .collect::<Vec<_>>()
            .await;

        let items: Vec<Item> = results
            .into_iter()
            .map(|r| match r {
                Ok(Ok(s)) => s,
                Ok(Err(e)) => {
                    println!("crawl error: {e}");
                    None
                },
                Err(e) => {
                    println!("join error: {e}");
                    None
                },
            })
            .flatten()
            .collect();

        let arc = Arc::new(items);

        items_cache.insert(sanitized_search_term.to_owned(), arc.clone()).await;
        arc
    }
}

fn sanitize_search_term(search_term: &str) -> String {
    search_term.trim().to_lowercase()
}