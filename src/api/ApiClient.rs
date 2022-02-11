use std::borrow::Borrow;
use std::collections::HashMap;
use reqwest::Response;
use anyhow::Result;
use serde_json::Value;
use std::sync::atomic::AtomicBool;
use tokio::sync::Mutex;

pub struct ApiClient {
    pub(crate) apikey: String,
    DEBUG_MODE: Mutex<AtomicBool>,
}

impl ApiClient {
    pub fn new(apikey: String) -> Self {
        Self {
            apikey,
            DEBUG_MODE: Mutex::new(AtomicBool::new(false)),
        }
    }

    pub async fn enable_debug_mode(mut self) -> Self {
        let mut temp = self.DEBUG_MODE.lock().await;
        *temp = AtomicBool::new(true);
        drop(temp);
        self
    }

    pub async fn get(&self, url: String) -> Result<Response> {
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


    pub async fn get_parse<T: serde::de::DeserializeOwned>(&self, url: String,) -> Result<T> {
        let resp = self.get(url).await?;

        if *self.DEBUG_MODE.lock().await.get_mut() {
            if let Ok(text) = resp.text().await {
                println!("GET {}", resp.url());
                println!("{:?}", text);
            }
        }

        Ok(())
    }
}

pub trait HandleValue {
    fn handle(value: Value) -> Value;
}

