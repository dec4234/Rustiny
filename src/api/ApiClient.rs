pub mod ApiClient {
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use reqwest::Response;
    use anyhow::Result;
    use serde_json::Value;

    pub struct ApiClient {
        pub(crate) apikey: String,
        DEBUG_MODE: bool,
    }

    impl ApiClient {
        pub fn new(apikey: String) -> Self {
            Self {
                apikey,
                DEBUG_MODE: false
            }
        }

        pub fn newWithDebug(apikey: String, debug: bool) -> Self {
            Self {
                apikey,
                DEBUG_MODE: debug
            }
        }

        pub async fn get(&self, url: String) -> Result<Response> {
            let client = reqwest::Client::new();
            let resp = client
                .get(url)
                .header("X-API-KEY", self.apikey.as_str())
                .send()
                .await?;

            if self.DEBUG_MODE {
                println!("{:?}", &resp);
                println!("test");
            }

            println!("test");

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

            if self.DEBUG_MODE {
                println!("{:?}", &resp);
                println!("test2");
            }

            Ok(resp)
        }
    }

    pub trait HandleValue {
        fn handle(value: Value) -> Value;
    }
}
