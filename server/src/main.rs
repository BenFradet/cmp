use std::{hash::RandomState, sync::Arc, time::Duration};

use domain::item::Item;
use moka::future::Cache;
use rate_limit::{rate_limit, Rate};
use reqwest::Client;
use warp::Filter;

use crate::solving::{solution::CachedSolution, solver::Solver};
use crate::filters::*;
use crate::handlers::search;

pub mod error;
pub mod filters;
pub mod handlers;
pub mod provider;
pub mod rate_limit;
pub mod selecting;
pub mod solving;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> () {

    let cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState> =
        Cache::builder()
            .max_capacity(10)
            .time_to_live(Duration::from_secs(3600))
            .build();

    let items_cache: Cache<String, Arc<Vec<Item>>, RandomState> =
        Cache::builder()
            .max_capacity(10000)
            .time_to_live(Duration::from_secs(3600))
            .build();

    let client = Client::new();

    let solver = Solver::create(&client, None).await.expect("could not initialize solver");

    let search = warp::get()
        .and(rate_limit(Rate::new(10, Duration::from_secs(60))))
        .and(warp::path!("api" / "v1" / "search"))
        .and(extract_q())
        .and(with_client(client))
        .and(with_items_cache(items_cache))
        .and(with_cookie_cache(cookie_cache))
        .and(with_solver(solver))
        .and_then(|
            _: (),
            search_term: String,
            client: Client,
            items_cache: Cache<String, Arc<Vec<Item>>, RandomState>,
            cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
            solver: Solver,
            | search(search_term, client, items_cache, cookie_cache, solver));

    let routes = search.recover(error::handle_rejection);
    println!("running at localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}