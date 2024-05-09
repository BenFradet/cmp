use reqwest::{header::HeaderValue, Client};
use serde_json::Value;

const FLARE_SOLVER: &'static str = "http://localhost:8191/v1";

pub struct Solver<'a> {
    client: &'a Client,
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

        Ok(Self {
            client,
            session_id,
        })
    }

    fn session_create_body(session_id: Option<String>) -> String {
        match session_id {
            Some(id) => serde_json::json!({"cmd": "sessions.create", "session": id}).to_string(),
            None => serde_json::json!({"cmd": "sessions.create"}).to_string(),
        }
    }
}