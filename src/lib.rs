use crate::api::DestinyAPI;

mod api;

#[tokio::test]
async fn run() {
    let api = DestinyAPI::ApiInterface::new("c57f52d5d071428fb8ff8684ba938212", true);

    api.client.get(String::from("https://www.bungie.net/Platform/Destiny2/3/Profile/4611686018468620320?components=308"));

    api.get_profile("4611686018468620320", DestinyAPI::STEAM).await;
}
