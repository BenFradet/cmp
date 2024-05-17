use std::{collections::HashMap, convert::Infallible, hash::RandomState, sync::Arc};

use domain::item::Item;
use moka::future::Cache;
use reqwest::Client;
use warp::{reject::Rejection, Filter};

use crate::{error::MissingQueryParam, solving::{solution::CachedSolution, solver::Solver}};

const QUERY_PARAM: &'static str = "q";

pub fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn with_solver(solver: Solver) -> impl Filter<Extract = (Solver,), Error = Infallible> + Clone {
    warp::any().map(move || solver.clone())
}

pub fn with_cookie_cache(
    cache: Cache<&'static str, Arc<CachedSolution>, RandomState>
) -> impl Filter<
    Extract = (Cache<&'static str, Arc<CachedSolution>, RandomState>,),
    Error = Infallible
> + Clone {
    warp::any().map(move || cache.clone())
}

pub fn with_items_cache(
    cache: Cache<String, Arc<Vec<Item>>, RandomState>
) -> impl Filter<
    Extract = (Cache<String, Arc<Vec<Item>>, RandomState>,),
    Error = Infallible
> + Clone {
    warp::any().map(move || cache.clone())
}

pub fn extract_q() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::query::<HashMap<String, String>>()
        .and_then(|p: HashMap<String, String>| async move {
            match p.get(QUERY_PARAM) {
                Some(search_term) => Ok(search_term.to_owned()),
                None => Err(warp::reject::custom(MissingQueryParam)),
            }
        })
}