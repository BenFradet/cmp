use std::{collections::HashMap, convert::Infallible, hash::RandomState, sync::Arc, time::Duration};

use domain::{item::Item, response::Response};
use error::MissingQueryParam;
use futures::stream::{self, StreamExt};
use moka::future::Cache;
use provider::Provider;
use reqwest::Client;
use warp::{reject::{self, Rejection}, reply::Reply, Filter};

use crate::solving::{solution::CachedSolution, solver::Solver};

pub mod error;
pub mod html_select;
pub mod provider;
pub mod search;
pub mod solving;

const PARALLEL_REQUESTS: usize = 3;
const QUERY_PARAM: &'static str = "q";
const DATE_FORMAT_STR: &'static str = "[year]-[month]-[day]-[hour]:[minute]:[second]";

#[tokio::main(flavor = "multi_thread")]
async fn main() -> () {

    let cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState> =
        Cache::builder()
            .max_capacity(10)
            .time_to_live(Duration::from_secs(3600 * 24))
            .build();

    let client = Client::new();

    let solver = Solver::create(&client, None).await.expect("could not initialize solver");

    let search = warp::get()
        .and(warp::path!("api" / "v1" / "search"))
        .and(extract_q())
        .and(with_client(client))
        .and_then(|search_term: String, client: Client| search(client, search_term));

    let routes = search.recover(error::handle_rejection);
    println!("running at localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// move to filters mod
fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
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

async fn search(
    client: Client,
    search_term: String,
) -> Result<impl Reply, Infallible> {
    println!("received: {search_term}");
    let res = fetch(client, &search_term).await;
    println!("results: {:?}", res);
    Ok(warp::reply::json(&Response { items: res }))
}

async fn fetch(
    client: Client,
    search_term: &str,
) -> Vec<Item> {
    let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];
    let providers_len = providers.len();

    let date_fmt = time::format_description::parse(&DATE_FORMAT_STR).unwrap();

    let results = stream::iter(providers)
        .map(|p| {
            // should be safe to clone since backed by an rc (to check)
            let client = client.clone();
            // corouting moves
            let s = search_term.to_owned();
            let fmt = date_fmt.clone();
            tokio::spawn(async move {
                p.crawl(&client, &s, fmt).await
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