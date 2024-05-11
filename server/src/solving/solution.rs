use std::{collections::HashMap, hash::RandomState, str::FromStr};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, COOKIE, USER_AGENT};
use serde::Deserialize;

use super::cookie::Cookie;

#[derive(Clone, Debug, Deserialize)]
pub struct Solution {
    headers: HashMap<String, String, RandomState>,
    user_agent: String,
    cookies: Vec<Cookie>,
}

impl Solution {
    pub fn header_map(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        for (k, v) in self.headers.into_iter() {
            headers.insert(HeaderName::from_str(&k).unwrap(), HeaderValue::from_str(&v).unwrap());
        }

        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());

        let cookies_str = self.cookies.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("; ");
        headers.insert(COOKIE, HeaderValue::from_str(&cookies_str).unwrap());

        headers
    }
}