use std::{collections::HashMap, hash::RandomState, str::FromStr};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, COOKIE, USER_AGENT};
use serde::Deserialize;

use super::cookie::Cookie;

#[derive(Debug, Deserialize)]
pub struct Solution {
    pub headers: HashMap<String, String, RandomState>,
    pub user_agent: String,
    pub cookies: Vec<Cookie>,
    pub response: String,
}

impl Solution {
    // todo: find a way without clones
    pub fn to_cached(&self) -> CachedSolution {
        CachedSolution {
            headers: self.headers.clone(),
            user_agent: self.user_agent.clone(),
            cookies: self.cookies.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CachedSolution {
    pub headers: HashMap<String, String, RandomState>,
    pub user_agent: String,
    pub cookies: Vec<Cookie>,
}

impl CachedSolution {
    pub fn header_map(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        for (k, v) in self.headers.iter() {
            headers.insert(HeaderName::from_str(&k).unwrap(), HeaderValue::from_str(&v).unwrap());
        }

        headers.insert(USER_AGENT, HeaderValue::from_str(&self.user_agent).unwrap());

        let cookies_str = self.cookies.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("; ");
        headers.insert(COOKIE, HeaderValue::from_str(&cookies_str).unwrap());

        headers
    }
}