use std::{hash::RandomState, sync::Arc, time::Duration};

use moka::future::Cache;
use reqwest::Client;
use warp::Filter;

use crate::solving::{solution::CachedSolution, solver::Solver};
use crate::filters::*;
use crate::handlers::search;

pub mod error;
pub mod filters;
pub mod handlers;
pub mod html_select;
pub mod provider;
pub mod solving;

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
        .and(with_cookie_cache(cookie_cache))
        .and(with_solver(solver))
        .and_then(|
            search_term: String,
            client: Client,
            cookie_cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
            solver: Solver,
            | search(search_term, client, cookie_cache, solver));

    let routes = search.recover(error::handle_rejection);
    println!("running at localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}