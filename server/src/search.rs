use reqwest::{header::HeaderValue, Client, IntoUrl};
use serde_json::Value;

const FLARE_SOLVER: &'static str = "http://localhost:8191/v1";

pub trait Search {
    async fn search<T>(&self, url: T, use_solver: bool) -> anyhow::Result<String> where T: IntoUrl {
        if use_solver {
            self.solver_search(url).await
        } else {
            self.direct_search(url).await
        }
    }

    async fn solver_search<T>(&self, url: T) -> anyhow::Result<String> where T: IntoUrl;
    async fn direct_search<T>(&self, url: T) -> anyhow::Result<String> where T: IntoUrl;

    fn bypass_req_body(url: &str) -> String {
        serde_json::json!({"cmd": "request.get", "url": url, "maxTimeout": 60000}).to_string()
    }
}

impl Search for Client {
    async fn direct_search<T>(&self, url: T) -> anyhow::Result<String> where T: IntoUrl {
        let resp = self.get(url).send().await?;
        resp.text().await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    async fn solver_search<T>(&self, url: T) -> anyhow::Result<String> where T: IntoUrl {
        let req_body = Self::bypass_req_body(url.as_str());
        let resp = self
            .post(FLARE_SOLVER)
            .body(req_body)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .send()
            .await?;
        let text = resp.text().await?;
        let json: Value = serde_json::from_str(&text)?;
        json["solution"]["response"]
            .as_str()
            .map(|s| s.to_owned())
            .ok_or(anyhow::anyhow!("solution.response could not be found in json"))
    }
}