use std::{collections::HashMap, convert::Infallible};

use domain::{provider::Provider, response::Response};
use error::MissingQueryParam;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use warp::{reject::{self, Rejection}, reply::Reply, Filter};

pub mod error;

const PARALLEL_REQUESTS: usize = 3;
const QUERY_PARAM: &'static str = "q";

#[tokio::main(flavor = "multi_thread")]
async fn main() -> () {
    let search = warp::get()
        .and(warp::path!("api" / "v1" / "search"))
        .and(extract_q())
        .and_then(|search_term: String| search(search_term));

    let routes = search.recover(error::handle_rejection);
    println!("running at localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn extract_q() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::query::<HashMap<String, String>>()
        .and_then(|p: HashMap<String, String>| async move {
            match p.get(QUERY_PARAM) {
                Some(search_term) => Ok(search_term.to_owned()),
                None => Err(reject::custom(MissingQueryParam)),
            }
        })
}

async fn search(search_term: String) -> Result<impl Reply, Infallible> {
    println!("received: {search_term}");
    let res = fetch(&search_term).await;
    println!("results: {:?}", res);
    Ok(warp::reply::json(&Response { results: res }))
}

async fn fetch(search_term: &str) -> Vec<String> {
    let client = Client::new();

    let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];
    let providers_len = providers.len();

    let results = stream::iter(providers)
        .map(|p| {
            // should be safe to clone since backed by an rc (to check)
            let client = client.clone();
            // corouting moves
            let s = search_term.to_owned();
            tokio::spawn(async move {
                p.crawl(&client, &s).await.unwrap_or("not found".to_owned())
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS)
        .take(providers_len)
        .collect::<Vec<_>>()
        .await;

    results
        .into_iter()
        .map(|r| match r {
            Ok(s) => Some(s),
            Err(e) => {
                println!("join error: {e}");
                None
            },
        })
        .flatten()
        .collect()
}