use crate::api::DestinyAPI::ApiInterface;
use crate::api::user::BungieUser::DestinyPlatform;
use crate::api::clan::Clan::Clan;

mod api;

async fn get_api() -> ApiInterface {
    ApiInterface::new("c57f52d5d071428fb8ff8684ba938212", true).await
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

#[tokio::test]
async fn get_clan_by_id() {
    let clan = Clan::get_by_id(get_api().await.client, 3074427).await.unwrap();

    println!("------Get Clan By Id------");
    print_clan(clan);
    println!();
}

#[tokio::test]
async fn get_clan_by_name() {
    let clan = Clan::get_by_name(get_api().await.client, "Heavenly Mayhem").await.unwrap();

    println!("------Get Clan By Name------");
    print_clan(clan);
    println!();
}

fn print_clan(clan: Clan) {
    println!("-------Clan Main Info-------");
    println!("Clan Id - {}", clan.id);
    println!("Clan Name - {}", clan.name);
    println!("Group Type - {}", clan.groupType);
    println!("Founder ID - {}", clan.founderId);
    println!("Description - {}\n", clan.description);
    println!("Member Count - {}", clan.memberCount);
    println!("Is Public - {}", clan.isPublic);
    println!("Is Public Topic Admin Only - {}", clan.isPublicTopicAdminOnly);
    println!("Motto - {}", clan.motto);
    println!("Allow Chat - {}", clan.allowChat);
    println!("Is Default Post Public - {}", clan.isDefaultPostPublic);

}