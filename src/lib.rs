use crate::api::DestinyAPI;
use crate::api::DestinyAPI::ApiInterface;
use crate::api::user::BungieUser::DestinyPlatform;

mod api;

async fn get_api() -> ApiInterface {
    ApiInterface::new("c57f52d5d071428fb8ff8684ba938212", true).await
}

#[tokio::test]
async fn run() {
    get_api().await.client.get(String::from("https://www.bungie.net/Platform/Destiny2/3/Profile/4611686018468620320?components=308")).await;
}

#[tokio::test]
async fn get_user() {
    let user = get_api().await.get_user("4611686018468620320", DestinyPlatform::Steam).await;
    println!("{}", user.unwrap().primary.id);
}