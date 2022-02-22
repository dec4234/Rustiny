use crate::api::DestinyAPI::ApiInterface;
use crate::api::user::BungieUser::{BungieUser, DestinyPlatform};
use crate::api::clan::Clan::Clan;

mod api;

async fn get_api() -> ApiInterface {
    ApiInterface::new("c57f52d5d071428fb8ff8684ba938212", true).await
}

#[tokio::test]
async fn get_user() {
    println!("-----Get User By ID-----");
    let user = get_api().await.get_user_by_id(String::from("4611686018468620320"), DestinyPlatform::Steam).await.unwrap();
    print_user(user);
}

#[tokio::test]
async fn get_users_by_name() {
    println!("-----Get Users By Name-----");
    println!();
    let list = BungieUser::get_users_with_name(&get_api().await.client, String::from("Ghost")).await.unwrap();

    for user in list {
        println!("User ID: {}", user.id);
    }
}

#[tokio::test]
async fn get_user_by_name_one() {
    println!("-----Get Users By Name But Only One Person-----\n");
    let list = BungieUser::get_users_with_name(&get_api().await.client, String::from("dec4234")).await.unwrap();

    for user in list {
        println!("User ID: {}", user.id);
    }
}

#[tokio::test]
async fn get_user_by_name_and_discriminator() {
    println!("-----Get User By Name And Discriminator-----");
    let user = BungieUser::get_user_by_name_and_discrim_with_platform(&get_api().await.client, String::from("dec4234#9904"), DestinyPlatform::All).await.unwrap();
    print_user(user);
}

#[tokio::test]
async fn test_name_splitting() {
    println!("Test Name Splitting - Failure Intended");
    let user = BungieUser::get_user_by_name_and_discrim_with_platform(&get_api().await.client, String::from("dec4234"), DestinyPlatform::All).await;

    if let Ok(user) = user {
        panic!("Method Returned OK instead of failing due to a lack of a specified discriminator");
    }
}

fn print_user(user: BungieUser) {
    println!();
    println!("--------Destiny Membership Info--------");
    println!("ID - {}", user.primary.id);
    println!("Platform Type - {}", user.primary.platform);
    if let Some(date) = user.primary.dateLastPlayed {
        println!("Last Played - {}", date);
    }

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
    println!("------Get Clan By Id------");
    let clan = Clan::get_by_id(get_api().await.client, 3074427).await.unwrap();

    print_clan(clan);
    println!();
}

#[tokio::test]
async fn get_clan_by_name() {
    println!("------Get Clan By Name------");
    let clan = Clan::get_by_name(get_api().await.client, "Heavenly Mayhem").await.unwrap();

    print_clan(clan);
    println!();
}

fn print_clan(clan: Clan) {
    println!();
    println!("-------Clan Main Info-------");
    println!("Clan Id - {}", clan.detail.id);
    println!("Clan Name - {}", clan.detail.name);
    println!("Group Type - {}", clan.detail.groupType);
    println!("Founder ID - {}", clan.detail.founderId);
    println!("Creation Date - {}", clan.detail.creationDate.unwrap());
    println!("Modification Date - {}", clan.detail.modificationDate.unwrap());

    println!("Description - {}\n", clan.detail.description);
    println!("Member Count - {}", clan.detail.memberCount);
    println!("Is Public - {}", clan.detail.isPublic);
    println!("Is Public Topic Admin Only - {}", clan.detail.isPublicTopicAdminOnly);
    println!("Motto - {}", clan.detail.motto);
    println!("Allow Chat - {}", clan.detail.allowChat);
    println!("Is Default Post Public - {}", clan.detail.isDefaultPostPublic);
    println!("Chat Security - {}", clan.detail.chatSecurity);
    println!("Locale - {}", clan.detail.locale);
    println!("Avatar Image Index - {}", clan.detail.avatarImageIndex);
    println!("Homepage - {}", clan.detail.homepage);
    println!("Membership Option - {}", clan.detail.membershipOption);
    println!("Default Publicity - {}", clan.detail.defaultPublicity);
    println!("Theme - {}", clan.detail.theme);
    println!("Avatar Path - {}", clan.detail.avatarPath);
    println!("Banner Path - {}", clan.detail.bannerPath);
    println!("Conversation ID - {}", clan.detail.conversationId);
    println!("Enable Invitation Messaging For Admins - {}", clan.detail.enableInvitationMessagingForAdmins);

    println!("Ban Expiration Date - {}", clan.detail.banExpireDate.unwrap());

    println!("Number of Allied IDs - {}", clan.alliedIds.len());
    println!("Alliance Status - {}", clan.allianceStatus);
    println!("Group Join Invite Count - {}", clan.groupJoinInviteCount);
    println!("Current User Memberships Inactive For Destiny - {}", clan.currentUserMembershipsInactiveForDestiny);

    println!();
    println!("-----Clan Features-----");
    println!("Maximum Members - {}", clan.detail.features.maximumMembers);
    println!("Maximum Memberships Of Group Type - {}", clan.detail.features.maximumMembershipsOfGroupType);
    println!("Capabilities - {}", clan.detail.features.capabilities);
    print!("Membership Types - [");
    for i in clan.detail.features.membershipTypes {
        print!("{}, ", i);
    }
    println!("]");
    println!("Invite Permissions Override - {}", clan.detail.features.invitePermissionOverride);
    println!("Update Culture Permission Override - {}", clan.detail.features.updateCulturePermissionOverride);
    println!("Host Guided Games Permission Override - {}", clan.detail.features.hostGuidedGamePermissionOverride);
    println!("Update Banner Permission Override - {}", clan.detail.features.updateBannerPermissionOverride);
    println!("Join Level - {}", clan.detail.features.joinLevel);
}