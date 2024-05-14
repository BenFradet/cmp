use std::{hash::RandomState, sync::Arc};

use moka::future::Cache;
use reqwest::{header::HeaderValue, Client, IntoUrl};
use serde_json::Value;

use super::{solution::CachedSolution, solver_response::SolverResponse};

const FLARE_SOLVER: &'static str = "http://localhost:8191/v1";

pub struct Solver {
    session_id: String,
}

impl Solver {
    pub async fn create(client: &Client, session_id: Option<String>) -> anyhow::Result<Self> {
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

        Ok(Self { session_id })
    }

    pub async fn solve<T>(
        &self,
        client: &Client,
        cache: &Cache<&'static str, Arc<CachedSolution>, RandomState>,
        provider_name: &'static str,
        url: T
    ) -> anyhow::Result<String> where T: IntoUrl {
        if let Some(solution) = cache.get(provider_name).await {
            let headers = solution.header_map();
            println!("{provider_name} found in cache, headers: {:?}", headers);

            let resp = client
                .get(url)
                .headers(headers)
                .send()
                .await?;
            resp
                .text()
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        } else {
            let body = self.solver_body(url.as_str());
            let resp = client
                .post(FLARE_SOLVER)
                .body(body)
                .header("Content-Type", HeaderValue::from_static("application/json"))
                .send()
                .await?;
            let text = resp.text().await?;
            let json: SolverResponse = serde_json::from_str(&text)?;

            let solution = json.solution;

            let cached_solution = solution.to_cached();
            println!("{provider_name} not found in cache, caching: {:?}", cached_solution);
            cache.insert(provider_name, Arc::new(cached_solution)).await;

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