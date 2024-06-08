use std::{convert::Infallible, time::Duration};

use serde_derive::Serialize;
use warp::{http::StatusCode, reject::{Reject, Rejection}, reply::Reply};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

const MISSING_QUERY_PARAM_MSG: &'static str = "No \"q\" query parameter specified";

#[derive(Debug)]
pub struct MissingQueryParam;

impl Reject for MissingQueryParam {}

#[derive(Debug)]
pub struct RateLimited {
    pub remaining_duration: Duration,
}

impl Reject for RateLimited {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let(code, message) = if err.is_not_found() {
        (
            StatusCode::NOT_FOUND,
            "NOT_FOUND".to_string(),
        )
    } else if let Some(MissingQueryParam) = err.find() {
        (
            StatusCode::BAD_REQUEST,
            MISSING_QUERY_PARAM_MSG.to_string(),
        )
    } else if let Some(RateLimited { remaining_duration }) = err.find::<RateLimited>() {
        let seconds = remaining_duration.as_secs();
        (
            StatusCode::TOO_MANY_REQUESTS,
            format!("Too many requests, rate limited for {seconds} seconds"),
        )
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "METHOD_NOT_ALLOWED".to_string(),
        )
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "UNHANDLED_REJECTION".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}