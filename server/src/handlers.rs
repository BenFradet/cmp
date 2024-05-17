use std::{convert::Infallible, hash::RandomState, sync::Arc};

use domain::{item::Item, response::Response};
use futures::stream::{self, StreamExt};
use moka::future::Cache;
use reqwest::Client;
use warp::reply::Reply;

use crate::{provider::Provider, solving::{solution::CachedSolution, solver::Solver}};

const PARALLEL_REQUESTS: usize = 3;
const DATE_FORMAT_STR: &'static str = "[year]-[month]-[day]-[hour]:[minute]:[second]";

pub async fn search(
    search_term: String,
    client: Client,
    cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
    solver: Solver,
) -> Result<impl Reply, Infallible> {
    println!("received: {search_term}");
    let res = fetch(&search_term, client, cookie_cache, solver).await;
    println!("results: {:?}", res);
    Ok(warp::reply::json(&Response { items: res }))
}

async fn fetch(
    search_term: &str,
    client: Client,
    cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
    solver: Solver,
) -> Vec<Item> {
    let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];
    let providers_len = providers.len();

    let date_fmt = time::format_description::parse(&DATE_FORMAT_STR).unwrap();

    let results = stream::iter(providers)
        .map(|p| {
            let client = client.clone();
            let cookie_cache = cookie_cache.clone();
            let solver = solver.clone();
            // corouting moves
            let s = search_term.to_owned();
            let fmt = date_fmt.clone();
            tokio::spawn(async move {
                p.crawl(&client, cookie_cache, solver, &s, fmt).await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS)
        .take(providers_len)
        .collect::<Vec<_>>()
        .await;

    results
        .into_iter()
        .map(|r| match r {
            Ok(Ok(s)) => Some(s),
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
        .collect()
}