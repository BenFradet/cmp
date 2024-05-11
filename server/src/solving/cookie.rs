use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Cookie {
    domain: String,
    expiry: Option<u64>,
    http_only: bool,
    name: String,
    path: String,
    same_site: String,
    secure: bool,
    value: String,
}

impl Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}