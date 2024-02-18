use super::Provider;
use anyhow::Context;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub struct SourceBin;

impl Provider for SourceBin {
    fn upload(content: String) -> anyhow::Result<String> {
        let body = Request::new(content);
        let res = Client::new()
            .post("https://sourceb.in/api/bins")
            .json(&body)
            .send()
            .context("Failed to send request")?;
        let res: Response = res.json().context("Failed to parse response")?;

        Ok(format!("https://sourceb.in/{}", res.key))
    }
}

#[derive(Serialize)]
struct Request {
    files: Vec<File>,
}

impl Request {
    fn new(content: String) -> Self {
        Self {
            files: vec![File { content }],
        }
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    key: String,
}

#[derive(Serialize)]
pub struct File {
    content: String,
}
