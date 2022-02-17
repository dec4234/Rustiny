use crate::api::DestinyAPI::ApiInterface;
use crate::api::user::BungieUser::DestinyPlatform;

mod api;

async fn get_api() -> ApiInterface {
    ApiInterface::new("c57f52d5d071428fb8ff8684ba938212", true).await
}

#[tokio::test]
async fn run() {
    get_api().await.client.get(String::from("https://www.bungie.net/Platform/Destiny2/3/Profile/4611686018468620320?components=308")).await;
    println!()
}

#[tokio::test]
async fn get_user() {
    let user = get_api().await.get_user("4611686018468620320", DestinyPlatform::Steam).await.unwrap();
    println!("--------Destiny Membership Info--------");
    println!("ID - {}", user.primary.id);
    println!("Platform Type - {}", user.primary.platform);

    println!("Platform Display Name - {}", user.primary.platform_display_name);
    println!("Cross Save Override - {}", user.primary.cross_save_override);

    println!("Global Display Name - {}", user.primary.global_display_name);
    println!("Name Discriminator - {}", user.primary.discriminator);

    println!("Is Public - {}", user.primary.is_public);
    println!("Is Overridden - {}", user.primary.is_overridden);
    println!("Is Cross Save Primary - {}", user.primary.is_cross_save_primary);

    print!("Applicable Membership Types - [");
    for i in user.primary.membership_types {
        print!(" {},", i);
    }
    println!(" ]\n");

    println!("--------BNet Membership Info--------");
    println!("Combined Name - {}", user.bnet_membership.combined_name);
    println!("Bnet Membership Display Name - {}", user.bnet_membership.display_name);
    println!("BNet Id - {}", user.bnet_membership.bnet_membership_id);
    println!("Bnet Iconpath - {}", user.bnet_membership.icon_path);
}