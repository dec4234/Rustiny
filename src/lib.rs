use std::borrow::Borrow;
use crate::api::activity::activity::{ActivityIdentifier, ActivityMode, PGCR, PgcrScraper};
use crate::api::DestinyAPI::ApiInterface;
use crate::api::user::BungieUser::{BungieUser, DestinyPlatform};
use crate::api::clan::Clan::Clan;
use crate::api::manifest::manifest::{Manifest, ManifestEntityType};
use crate::api::user::DestinyCharacter::DestinyCharacter;
use anyhow::Result;

pub mod api;

struct Tester {
    interface: ApiInterface,
    user: Option<BungieUser>,
    character: Option<DestinyCharacter>,
    clan: Option<Clan>,
}

impl Tester {
    async fn new() -> Self {
        let mut test = Self {
            interface: ApiInterface::new("c57f52d5d071428fb8ff8684ba938212", false).await,
            user: None,
            character: None,
            clan: None,
        };

        test.user = Some(test.interface.get_user_by_id("4611686018468620320".to_string(), DestinyPlatform::Steam).await.unwrap());
        // test.character = Some(test.user.clone().unwrap().get_characters(&test.interface.client).await.unwrap().get(0).unwrap().clone());

        test
    }

    fn get_user(&self) -> BungieUser {
        self.user.clone().unwrap()
    }

    fn get_character(&self) -> DestinyCharacter {
        self.character.clone().unwrap()
    }

    async fn test_all(&self, scraper: &PgcrScraper) {
        // self.activity_history(scraper).await;
        self.get_unknown_activity_hashes(scraper).await;
    }

    async fn activity_history(&self, scraper: &PgcrScraper) {
        println!("Acitivty History Reports - {}", scraper.get_activity_history(self.get_user(), ActivityMode::Raid).await.unwrap().len());
    }

    async fn get_unknown_activity_hashes(&self, scraper: &PgcrScraper) {
        for ah in scraper.get_activity_history(self.get_user(), ActivityMode::ScoredNightfall).await.unwrap() {
            if let None = ActivityIdentifier::from_identifier(format!("{}", ah.activityDetails.referenceId)) {
                println!("{}", ah.activityDetails.referenceId);
            }
        }
    }
}

#[tokio::test]
async fn test_tester_items() {
    let test = Tester::new().await;
    let scraper = PgcrScraper::new(&test.interface.client.clone().await).await;

    test.test_all(&scraper).await;
}

#[tokio::test]
#[ignore]
async fn test_hashes() {
    let man = Manifest::new(get_api().await.client);

    let vec = vec!["3849697856", "887176543", "3289589202", "3718330161", "272852450", "1034003646", "1282886582", "936308438"];

    for s in vec {
        println!("{} = {}", s, man.manifest_get(ManifestEntityType::ACTIVITY, String::from(s)).await.unwrap());
    }
}

async fn get_api() -> ApiInterface {
    ApiInterface::new("c57f52d5d071428fb8ff8684ba938212", true).await
}

#[tokio::test]
async fn get_user() {
    println!("-----Get User By ID-----");
    let user = get_api().await.get_user_by_id(String::from("4611686018468620320"), DestinyPlatform::Steam).await.unwrap();
    print_user(&user);
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
async fn get_user_by_name_and_discriminator_and_characters() {
    println!("-----Get User By Name And Discriminator-----");
    let user = BungieUser::get_user_by_name_and_discrim_with_platform(&get_api().await.client, String::from("dec4234#9904"), DestinyPlatform::All).await.unwrap();
    print_user(&user);

    println!("\n-----Get Characters-----");
    get_characters(&user).await;
}

async fn get_characters(user: &BungieUser) {
    for c in user.get_characters(&get_api().await.client).await.unwrap() {
        println!("\n-----Character-----");
        print_character(c);
    }
}

fn print_character(c: DestinyCharacter) {
    println!("ID - {}", c.characterId);
    println!("Date Last Played - {}", c.dateLastPlayed.unwrap());
    println!("Light Level - {}", c.light);
}

#[tokio::test]
async fn test_name_splitting() {
    println!("Test Name Splitting");
    let user = BungieUser::get_user_by_name_and_discrim_with_platform(&get_api().await.client, String::from("dec4234"), DestinyPlatform::All).await;

    if let Ok(user) = user {
        panic!("Method Returned OK instead of failing due to a lack of a specified discriminator");
    }
}

fn print_user(user: &BungieUser) {
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
    if let Some(overriden) = user.primary.is_overridden {
        println!("Is Overridden - {}", overriden);
    }
    if let Some(primary) = user.primary.is_cross_save_primary {
        println!("Is Cross Save Primary - {}", primary);
    }


    print!("Applicable Membership Types - [");
    for i in &user.primary.membership_types {
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
    println!("Creation Date - {}", clan.detail.creationDate);
    println!("Modification Date - {}", clan.detail.modificationDate);

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

    println!("Ban Expiration Date - {}", clan.detail.banExpireDate);

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

#[tokio::test]
#[ignore]
async fn clan_weekly_rewards() {
    let clan = Clan::get_by_id(get_api().await.client, 3074427).await.unwrap();

    let man = Manifest::new(get_api().await.client);

    let rewards = clan.get_weekly_rewards(&get_api().await.client).await.unwrap();
    let list = rewards.rewards.get(0).unwrap();

    for e in list.entries.clone() {
        println!("{}", man.manifest_reward(rewards.milestoneHash, e.rewardEntryHash).await.unwrap().displayProperties.name);
    }
}

#[tokio::test]
async fn clan_members() {
    let clan = Clan::get_by_id(get_api().await.client, 3074427).await.unwrap();

    for m in clan.get_members(&get_api().await.client).await.unwrap() {
        println!("{} - {}", m.destinyUserInfo.LastSeenDisplayName.unwrap(), m.isOnline.unwrap());
    }
}

#[tokio::test]
async fn get_founder() {
    let clan = Clan::get_by_id(get_api().await.client, 3074427).await.unwrap();
    let founder = clan.founder;

    println!("{} - {}", founder.destinyUserInfo.global_display_name, founder.joinDate);
}

#[tokio::test]
pub async fn test_pgcr_one() {
    let pgcr = PgcrScraper::new(&get_api().await.client).await.get_pgcr(1).await.unwrap();
    print_pgcr(&pgcr);
}

#[tokio::test]
pub async fn test_pgcr_trials() {
    let pgcr = PgcrScraper::new(&get_api().await.client).await.get_pgcr(9496960718).await.unwrap();
    print_pgcr(&pgcr);
}

#[tokio::test]
pub async fn test_pgcr_votd() {
    let pgcr = PgcrScraper::new(&get_api().await.client).await.get_pgcr(10405562745).await.unwrap();
    print_pgcr(&pgcr);
}

fn print_pgcr(pgcr: &PGCR) {
    println!("ID - {}", pgcr.activityDetails.instanceId);
    println!("Date - {}", pgcr.period);
    println!("Starting Phase Index - {}", pgcr.startingPhaseIndex);
    println!("Was Started From The Beginning - {}", pgcr.activityWasStartedFromBeginning);


    for entry in pgcr.entries.clone() {
        println!("\n------Entry-----");
        println!("Username - {}#{}", entry.player.destinyUserInfo.global_display_name, entry.player.destinyUserInfo.discriminator);
        println!("Character ID - {}", entry.characterId);
        println!("Standing - {}", entry.standing);
        println!("---Values---");
        println!("Player Count - {}", entry.values.playerCount.basic.value);
        println!("Completed - {}", entry.values.completed.basic.displayValue);
        println!("Completion Reason - {}", entry.values.completionReason.basic.displayValue);
        println!("Start Seconds - {}", entry.values.startSeconds.basic.value);
        println!("Activity Duration - {}", entry.values.activityDurationSeconds.basic.displayValue);
    }
}

#[tokio::test]
#[ignore]
pub async fn manifest_test() {
    let man = Manifest::new(get_api().await.client);

    let vec = vec!["2318521576", "3711627564", "3725993747", "2630091891", "4134816102"];

    for s in vec {
        println!("{} = {}", s, man.manifest_get(ManifestEntityType::ACTIVITY, String::from(s)).await.unwrap());
    }

}