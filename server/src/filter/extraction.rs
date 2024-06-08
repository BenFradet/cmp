use std::collections::HashMap;

use warp::{reject::Rejection, Filter};

use crate::error::MissingQueryParam;

const QUERY_PARAM: &'static str = "q";

pub fn extract_q() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::query::<HashMap<String, String>>()
        .and_then(|p: HashMap<String, String>| async move {
            match p.get(QUERY_PARAM) {
                Some(search_term) => Ok(search_term.to_owned()),
                None => Err(warp::reject::custom(MissingQueryParam)),
            }
        })
}