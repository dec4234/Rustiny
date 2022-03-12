use std::sync::{Arc};
use reqwest::Response;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use crate::api::ApiClient::ApiClient;
use anyhow::Result;
use tokio::sync::{Mutex, MutexGuard};
use crate::api::user::BungieUser::{DestinyPlatform, BungieUser};

pub struct ApiInterface {
    pub client: ApiClient,
}

impl ApiInterface {
    pub async fn new(apikey: &str, debug: bool) -> Self {
        let mut client = ApiClient::new(apikey);

        if debug {
            client = client.enable_debug_mode().await;
        }

        Self {
            client,
        }
    }

    /// Get a user using their id and platform type
    ///
    /// For example
    /// ```rust
    /// let interface = ApiInterface::new("YOUR API KEY HERE", true).await;
    /// let user = interface.get_user_by_id(String::from("4611686018468620320"), DestinyPlatform::Steam).await.unwrap();
    /// println!("{}", user.primary.id);
    /// ```
    pub async fn get_user_by_id(&self, id: String, platform: DestinyPlatform) -> Result<BungieUser> {
        BungieUser::get_user_by_id(&self.client, id, platform).await
    }
}

// Other
pub const URL_BASE: &str = "https://www.bungie.net/Platform";

