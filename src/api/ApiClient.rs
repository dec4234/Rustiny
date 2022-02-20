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

    pub async fn enable_debug_mode(self) -> Self {
        let mut temp = self.DEBUG_MODE.lock().await;
        *temp = AtomicBool::new(true);
        drop(temp);
        self
    }

    pub async fn is_debug_enabled(&self) -> bool {
        *self.DEBUG_MODE.lock().await.get_mut()
    }

    pub async fn get(&self, url: String) -> Result<String> {
        let client = reqwest::Client::new();
        let resp = client
            .get(url.clone())
            .header("X-API-KEY", self.apikey.as_str())
            .send()
            .await?;

        let text = resp.text().await?;

        if self.is_debug_enabled().await {
            println!("GET {}", url);
            println!("{}", text.clone());
        }


        Ok(text)
    }

    pub async fn getWithParams(&self, url: &str, params: HashMap<&str, &str>) -> Result<Response> {
        let client = Client::new();
        let resp = client
            .get(url)
            .header("X-API-KEY", self.apikey.as_str())
            .query(&params)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn post(&self, url: String, body: String) -> Result<String> {
        let client = Client::new();
        let resp = client
            .post(url.clone())
            .body(body.clone())
            .header("X-API-KEY", self.apikey.as_str())
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


    pub async fn get_parse<T: DeserializeOwned>(&self, url: String,) -> Result<T> {
        let text = self.get(url.clone()).await?;

        if self.is_debug_enabled().await {
            println!("GET {}", url);
            println!("{}", &text);
        }

        let r = serde_json::from_str::<T>(text.as_str())?;

        Ok(r)
    }

    pub async fn post_parse<T: DeserializeOwned>(&self, url: String, body: String,) -> Result<T> {
        let text = self.post(url, body).await?;

        let r = serde_json::from_str::<T>(text.as_str())?;

        Ok(r)
    }
}

pub trait HandleValue {
    fn handle(value: Value) -> Value;
}

fn encode_url(url: String) -> String {
    url.replace(" ", "%20")
}
