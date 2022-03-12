use std::borrow::Borrow;
use std::collections::HashMap;
use reqwest::{Client, Response};
use anyhow::Result;
use serde_json::Value;
use std::sync::atomic::AtomicBool;
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;

pub struct ApiClient {
    pub(crate) apikey: String,
    DEBUG_MODE: Mutex<AtomicBool>,
}

impl ApiClient {
    pub fn new(apikey: &str) -> Self {
        Self {
            apikey: String::from(apikey),
            DEBUG_MODE: Mutex::new(AtomicBool::new(false)),
        }
    }

    /// Enables Debug Mode
    ///
    /// Prints all requests and their responses as they come through
    /// usually only needed for development of the API but may be useful
    /// to someone wanting to learn the inner-workings of the system.
    pub async fn enable_debug_mode(self) -> Self {
        let mut temp = self.DEBUG_MODE.lock().await;
        *temp = AtomicBool::new(true);
        drop(temp);
        self
    }

    pub async fn is_debug_enabled(&self) -> bool {
        *self.DEBUG_MODE.lock().await.get_mut()
    }

    /// Clones the ApiClient
    pub async fn clone(&self) -> Self {
        Self {
            apikey: self.apikey.clone(),
            DEBUG_MODE: Mutex::new(AtomicBool::new(self.is_debug_enabled().await)),
        }
    }

    pub async fn get(&self, url: String) -> Result<String> {
        self.get_params(url, HashMap::new()).await
    }

    pub async fn get_params(&self, url: String, map: HashMap<&str, &str>) -> Result<String> {
        let client = reqwest::Client::new();
        let resp = client
            .get(url.clone())
            .header("X-API-KEY", self.apikey.as_str())
            .query(&map)
            .send()
            .await?;

        let text = resp.text().await?;

        if self.is_debug_enabled().await {
            println!("GET {}", url);
            println!("{}", text.clone());
        }


        Ok(text)
    }

    pub async fn get_parse<T: DeserializeOwned>(&self, url: String,) -> Result<T> {
        let text = self.get(url.clone()).await?;

        let r = serde_json::from_str::<T>(text.as_str())?;

        Ok(r)
    }

    pub async fn get_parse_params<T: DeserializeOwned>(&self, url: String, map: HashMap<&str, &str>) -> Result<T> {
        let text = self.get_params(url, map).await?;

        Ok(serde_json::from_str::<T>(text.as_str())?)
    }

    pub async fn post(&self, url: String, body: String) -> Result<String> {
        self.post_params(url, body, HashMap::new()).await
    }

    pub async fn post_params(&self, url: String, body: String, map: HashMap<&str, &str>) -> Result<String> {
        let client = Client::new();
        let resp = client
            .post(url.clone())
            .body(body.clone())
            .header("X-API-KEY", self.apikey.as_str())
            .query(&map)
            .send()
            .await?;

        let text = resp.text().await?;

        if self.is_debug_enabled().await {
            println!("POST {}", url);
            println!("Body - {}", body);
            println!("{}", text.clone());
        }

        Ok(text)
    }

    pub async fn post_parse<T: DeserializeOwned>(&self, url: String, body: String,) -> Result<T> {
        let text = self.post(url, body).await?;

        let r = serde_json::from_str::<T>(text.as_str())?;

        Ok(r)
    }

    pub async fn post_parse_params<T: DeserializeOwned>(&self, url: String, body: String, map: HashMap<&str, &str>) -> Result<T> {
        let text = self.post_params(url, body, map).await?;

        Ok(serde_json::from_str::<T>(text.as_str())?)
    }
}

pub trait HandleValue {
    fn handle(value: Value) -> Value;
}

fn encode_url(url: String) -> String {
    url.replace(" ", "%20")
}
