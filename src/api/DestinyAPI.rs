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

    pub async fn get_user_by_id(&self, id: String, platform: DestinyPlatform) -> Result<BungieUser> {
        BungieUser::get_user_by_id(&self.client, id, platform).await
    }
    /*
    pub async fn get_profile(&self, bungieID: &str, membershipType: u8) -> Option<BungieUser> {
        let response = self.client.get(URL_BASE.to_owned() + "/" + membershipType.to_string().as_str() + "/Profile/" + bungieID + "?components=100");

        let resp = response.await.unwrap();
        let val: Value = serde_json::from_str(resp.text().await.unwrap().as_str()).unwrap();
        let val = &val["Response"]["profile"]["data"]["userInfo"];
        println!("{}", &val.to_string().as_str());
        let userInfo: BungieUser = serde_json::from_value(val.clone()).unwrap();


        // ["Response"]["profile"]["data"]["userInfo"]
        Some(userInfo)
    }
     */
}

/*
#[derive(Deserialize)]
pub struct BungieUser {

    #[serde(rename = "membershipId")]
    pub bungieID: String,
    #[serde(rename = "bungieGlobalDisplayName")]
    pub global_display_name: String,
    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub nameDiscriminator: u16,
    pub displayName: String,

}
 */

// Membership Types
pub static STEAM: u8 = 3;

// Component Strings
static PROFILES: &str = "Profiles";
static CHARACTERS: &str = "Characters";

// Other
pub const URL_BASE: &str = "https://www.bungie.net/Platform";

