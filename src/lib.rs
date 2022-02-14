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
    let user = get_api().await.get_user("4611686018468620320", DestinyPlatform::Steam).await.unwrap();
    println!("--------Destiny Membership Info--------");
    println!("ID - {}", user.primary.id);
    println!("Plaform Type - {}", user.primary.platform);

    println!("Platform Display Name - {}", user.primary.platformDisplayName);
    println!("Cross Save Override - {}", user.primary.crossSaveOverride);

    println!("Global Display Name - {}", user.primary.globalDisplayName);
    println!("Name Discriminator - {}", user.primary.discriminator);

    println!("Is Public - {}", user.primary.isPublic);
    println!("Is Overridden - {}", user.primary.isOverridden);
    println!("Is Cross Save Primary - {}", user.primary.isCrossSavePrimary);

    print!("Applicable Membership Types - [");
    for i in user.primary.membershipTypes {
        print!(" {},", i);
    }
    println!(" ]\n");

    println!("--------BNet Membership Info--------");
    println!("Combined Name - {}", user.bnetMembership.combined_name);
}