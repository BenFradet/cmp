use std::error::Error;

use reqwest::IntoUrl;

pub trait Price {
    async fn price<T>(&self, url: T) -> Result<String, Box<dyn Error>> where T: IntoUrl;
}