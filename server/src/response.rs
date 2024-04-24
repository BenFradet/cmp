use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub results: Vec<String>,
}