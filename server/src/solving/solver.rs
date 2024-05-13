use std::{hash::RandomState, sync::Arc, time::Duration};

use moka::future::Cache;
use reqwest::{header::HeaderValue, Client, IntoUrl};
use serde_json::Value;

use super::{solution::CachedSolution, solver_response::SolverResponse};

const FLARE_SOLVER: &'static str = "http://localhost:8191/v1";

pub struct Solver<'a> {
    client: &'a Client,
    cache: Cache<&'static str, Arc<CachedSolution>, RandomState>,
    session_id: String,
}

impl<'a> Solver<'a> {
    pub async fn create(client: &'a Client, session_id: Option<String>) -> anyhow::Result<Self> {
        let body = Self::session_create_body(session_id);
        let resp = client
            .post(FLARE_SOLVER)
            .body(body)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .send()
            .await?;
        let text = resp.text().await?;
        let json: Value = serde_json::from_str(&text)?;
        // there is an end timestamp but it doesn't seem to be trustworthy
        let session_id = json["session"]
            .as_str()
            .map(|s| s.to_owned())
            .ok_or(anyhow::anyhow!("session could not be found in json"))?;

        let cache = Cache::builder()
            .max_capacity(10)
            .time_to_live(Duration::from_secs(3600 * 24))
            .build();

        Ok(Self {
            client,
            cache,
            session_id,
        })
    }

    pub async fn solve<T>(
        &self,
        provider_name: &'static str,
        url: T
    ) -> anyhow::Result<String> where T: IntoUrl {
        if let Some(solution) = self.cache.get(provider_name).await {
            let resp = self
                .client
                .get(url)
                .headers(solution.header_map())
                .send()
                .await?;
            resp
                .text()
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        } else {
            let body = self.solver_body(url.as_str());
            let resp = self
                .client
                .post(FLARE_SOLVER)
                .body(body)
                .header("Content-Type", HeaderValue::from_static("application/json"))
                .send()
                .await?;
            let text = resp.text().await?;
            let json: SolverResponse = serde_json::from_str(&text)?;

            let solution = json.solution;

            let cached_solution = solution.to_cached();
            self.cache.insert(provider_name, Arc::new(cached_solution)).await;

            Ok(solution.response)
        }
    }

    fn session_create_body(session_id: Option<String>) -> String {
        match session_id {
            Some(id) => serde_json::json!({"cmd": "sessions.create", "session": id}).to_string(),
            None => serde_json::json!({"cmd": "sessions.create"}).to_string(),
        }
    }

    fn solver_body(&self, url: &str) -> String {
        serde_json::json!({
            "cmd": "request.get", 
            "url": url, 
            "maxTimeout": 30000,
            "session": self.session_id,
        }).to_string()
    }
}