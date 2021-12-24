use std::collections::HashMap;
use reqwest::Response;
use anyhow::Result;
use serde_json::Value;

pub struct ApiClient {
    pub(crate) apikey: String,
}

impl ApiClient {
    pub async fn get(&self, url: &str) -> Result<Response> {
        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header("X-API-KEY", self.apikey.as_str())
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn getWithParams(&self, url: &str, params: HashMap<&str, &str>) -> Result<Response> {
        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header("X-API-KEY", self.apikey.as_str())
            .query(&params)
            .send()
            .await?;

        Ok(resp)
    }
}

pub trait HandleValue {
    fn handle(value: Value) -> Value;
}