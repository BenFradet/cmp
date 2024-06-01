use std::convert::Infallible;

use serde_derive::Serialize;
use warp::{http::StatusCode, reject::{Reject, Rejection}, reply::Reply};

use crate::rate_limit::RateLimited;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

const MISSING_QUERY_PARAM_MSG: &'static str = "No \"q\" query parameter specified";

#[derive(Debug)]
pub struct MissingQueryParam;

impl Reject for MissingQueryParam {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_string();
    } else if let Some(MissingQueryParam) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = MISSING_QUERY_PARAM_MSG.to_string();
    } else if let Some(RateLimited { remaining_duration }) = err.find::<RateLimited>() {
        code = StatusCode::TOO_MANY_REQUESTS;
        let seconds = remaining_duration.as_secs();
        message = format!("Too many requests, rate limited for {seconds} seconds");
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}