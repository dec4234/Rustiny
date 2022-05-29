use crate::api::ApiClient::ApiClient;
use anyhow::Result;

// https://crates.io/crates/rustls
// https://github.com/rustls/hyper-rustls/blob/main/examples/server.rs
pub struct OAuthClient {
    pub client: ApiClient,
}

impl OAuthClient {

}

pub trait NewTokenEvent {
    fn set_access_token(token: String);

    fn set_refresh_token(token: String);
}

pub trait GetOldTokens {
    fn get_access_token() -> Result<Option<String>>;

    fn get_refresh_token() -> Result<Option<String>>;
}