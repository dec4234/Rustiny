use reqwest::Response;
use crate::api::DestinyAPI;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use crate::api::ApiClient::ApiClient;
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Clan {
    #[serde(rename = "groupId")]
    pub id: String,
    pub name: String,
    pub groupType: u8,
    #[serde(rename = "membershipIdCreated")]
    pub founderId: String,
    #[serde(rename = "about")]
    pub description: String,
    pub tags: Vec<String>,
    pub memberCount: u8,
    pub isPublic: bool,
    pub isPublicTopicAdminOnly: bool,
    pub motto: String,
    pub allowChat: bool,
    pub isDefaultPostPublic: bool,
    pub chatSecurity: u8,
    pub locale: String,
    pub avatarImageIndex: u8,
    pub homepage: u8,
    pub membershipOption: u8,
    pub defaultPublicity: u8,
    pub theme: String,
    pub avatarPath: String,
    pub bannerPath: String,
}

impl Clan {
    pub async fn get_by_id(apiClient: ApiClient, id: i32) -> Result<Self> {
        let clan = apiClient.get(format!("{base}/GroupV2/{groupId}/", base = DestinyAPI::URL_BASE, groupId = id)).await?;

        Ok(Clan::from_string_response(clan)?)
    }

    pub async fn get_by_name(apiClient: ApiClient, name: &str) -> Result<Self> {
        let clan = apiClient.get(format!("{base}/GroupV2/Name/{groupName}/{groupType}/", base = DestinyAPI::URL_BASE, groupName = name, groupType = 1)).await?;

        Ok(Clan::from_string_response(clan)?)
    }

    fn from_string_response(response: String) -> Result<Self> {
        let val: Value = serde_json::from_str(response.as_str())?;

        Ok(serde_json::from_value::<Clan>(val["Response"]["detail"].clone())?)
    }
}

/*
/Platform/GroupV2/3074427/

{
  "Response": {
    "detail": {
      "groupId": "3074427",
      "name": "Heavenly Mayhem",
      "groupType": 1,
      "membershipIdCreated": "17506516",
      "creationDate": "2018-04-05T18:10:58.836Z",
      "modificationDate": "2021-05-13T11:40:48.856Z",
      "about": "ALL PROSPECTIVE MEMBERS MUST JOIN THE DISCORD: https://discord.gg/SnmRCpJ\n\nHeavenly Mayhem [HeM] is a North American based clan who play Destiny 2. We are an active community seeking to establish a chill and cool group of people who like raiding and hanging out. We run raid sherpa runs, divinity runs and experienced only runs.\n\nAll applications are ran through discord, if you do not apply in the discord and request to join on the bungie website, your request will be ignored or rejected.",
      "tags": [],
      "memberCount": 78,
      "isPublic": true,
      "isPublicTopicAdminOnly": false,
      "motto": "Agents of the Last Safe City on Earth",
      "allowChat": true,
      "isDefaultPostPublic": false,
      "chatSecurity": 0,
      "locale": "en",
      "avatarImageIndex": 0,
      "homepage": 0,
      "membershipOption": 0,
      "defaultPublicity": 2,
      "theme": "Group_Community1",
      "bannerPath": "/img/Themes/Group_Community1/struct_images/group_top_banner.jpg",
      "avatarPath": "/img/profile/avatars/group/defaultGroup.png",
      "conversationId": "38311024",
      "enableInvitationMessagingForAdmins": false,
      "banExpireDate": "2001-01-01T00:00:00Z",
      "features": {
        "maximumMembers": 100,
        "maximumMembershipsOfGroupType": 1,
        "capabilities": 31,
        "membershipTypes": [
          1,
          2,
          3,
          5
        ],
        "invitePermissionOverride": true,
        "updateCulturePermissionOverride": false,
        "hostGuidedGamePermissionOverride": 1,
        "updateBannerPermissionOverride": false,
        "joinLevel": 1
      },
      "clanInfo": {
        "d2ClanProgressions": {
          "584850370": {
            "progressionHash": 584850370,
            "dailyProgress": 600000,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 600000,
            "level": 6,
            "levelCap": 6,
            "stepIndex": 6,
            "progressToNextLevel": 0,
            "nextLevelAt": 0
          },
          "1273404180": {
            "progressionHash": 1273404180,
            "dailyProgress": 0,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 0,
            "level": 1,
            "levelCap": 6,
            "stepIndex": 1,
            "progressToNextLevel": 0,
            "nextLevelAt": 1
          },
          "3381682691": {
            "progressionHash": 3381682691,
            "dailyProgress": 0,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 0,
            "level": 1,
            "levelCap": 6,
            "stepIndex": 1,
            "progressToNextLevel": 0,
            "nextLevelAt": 1
          },
          "3759191272": {
            "progressionHash": 3759191272,
            "dailyProgress": 0,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 0,
            "level": 1,
            "levelCap": 6,
            "stepIndex": 1,
            "progressToNextLevel": 0,
            "nextLevelAt": 1
          }
        },
        "clanCallsign": "HeM",
        "clanBannerData": {
          "decalId": 4142223388,
          "decalColorId": 3379387803,
          "decalBackgroundColorId": 3585526349,
          "gonfalonId": 1473910866,
          "gonfalonColorId": 2157636322,
          "gonfalonDetailId": 1698031298,
          "gonfalonDetailColorId": 4078567632
        }
      }
    },
    "founder": {
      "memberType": 5,
      "isOnline": false,
      "lastOnlineStatusChange": "1644181974",
      "groupId": "3074427",
      "destinyUserInfo": {
        "LastSeenDisplayName": "dec4234",
        "LastSeenDisplayNameType": 3,
        "iconPath": "/img/theme/bungienet/icons/steamLogo.png",
        "crossSaveOverride": 0,
        "applicableMembershipTypes": [
          3
        ],
        "isPublic": false,
        "membershipType": 3,
        "membershipId": "4611686018468620320",
        "displayName": "dec4234",
        "bungieGlobalDisplayName": "dec4234",
        "bungieGlobalDisplayNameCode": 9904
      },
      "bungieNetUserInfo": {
        "supplementalDisplayName": "dec4234#9904",
        "iconPath": "/img/profile/avatars/cc14.jpg",
        "crossSaveOverride": 0,
        "isPublic": false,
        "membershipType": 254,
        "membershipId": "17506516",
        "displayName": "dec4234",
        "bungieGlobalDisplayName": "dec4234",
        "bungieGlobalDisplayNameCode": 9904
      },
      "joinDate": "2019-12-02T22:23:19Z"
    },
    "alliedIds": [],
    "allianceStatus": 0,
    "groupJoinInviteCount": 0,
    "currentUserMembershipsInactiveForDestiny": false,
    "currentUserMemberMap": {},
    "currentUserPotentialMemberMap": {}
  },
  "ErrorCode": 1,
  "ThrottleSeconds": 0,
  "ErrorStatus": "Success",
  "Message": "Ok",
  "MessageData": {}
}
 */

/*
/Platform/GroupV2/Name/Heavenly%20Mayhem/1/

{
  "Response": {
    "detail": {
      "groupId": "3074427",
      "name": "Heavenly Mayhem",
      "groupType": 1,
      "membershipIdCreated": "17506516",
      "creationDate": "2018-04-05T18:10:58.836Z",
      "modificationDate": "2021-05-13T11:40:48.856Z",
      "about": "ALL PROSPECTIVE MEMBERS MUST JOIN THE DISCORD: https://discord.gg/SnmRCpJ\n\nHeavenly Mayhem [HeM] is a North American based clan who play Destiny 2. We are an active community seeking to establish a chill and cool group of people who like raiding and hanging out. We run raid sherpa runs, divinity runs and experienced only runs.\n\nAll applications are ran through discord, if you do not apply in the discord and request to join on the bungie website, your request will be ignored or rejected.",
      "tags": [],
      "memberCount": 78,
      "isPublic": true,
      "isPublicTopicAdminOnly": false,
      "motto": "Agents of the Last Safe City on Earth",
      "allowChat": true,
      "isDefaultPostPublic": false,
      "chatSecurity": 0,
      "locale": "en",
      "avatarImageIndex": 0,
      "homepage": 0,
      "membershipOption": 0,
      "defaultPublicity": 2,
      "theme": "Group_Community1",
      "bannerPath": "/img/Themes/Group_Community1/struct_images/group_top_banner.jpg",
      "avatarPath": "/img/profile/avatars/group/defaultGroup.png",
      "conversationId": "38311024",
      "enableInvitationMessagingForAdmins": false,
      "banExpireDate": "2001-01-01T00:00:00Z",
      "features": {
        "maximumMembers": 100,
        "maximumMembershipsOfGroupType": 1,
        "capabilities": 31,
        "membershipTypes": [
          1,
          2,
          3,
          5
        ],
        "invitePermissionOverride": true,
        "updateCulturePermissionOverride": false,
        "hostGuidedGamePermissionOverride": 1,
        "updateBannerPermissionOverride": false,
        "joinLevel": 1
      },
      "clanInfo": {
        "d2ClanProgressions": {
          "584850370": {
            "progressionHash": 584850370,
            "dailyProgress": 600000,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 600000,
            "level": 6,
            "levelCap": 6,
            "stepIndex": 6,
            "progressToNextLevel": 0,
            "nextLevelAt": 0
          },
          "1273404180": {
            "progressionHash": 1273404180,
            "dailyProgress": 0,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 0,
            "level": 1,
            "levelCap": 6,
            "stepIndex": 1,
            "progressToNextLevel": 0,
            "nextLevelAt": 1
          },
          "3381682691": {
            "progressionHash": 3381682691,
            "dailyProgress": 0,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 0,
            "level": 1,
            "levelCap": 6,
            "stepIndex": 1,
            "progressToNextLevel": 0,
            "nextLevelAt": 1
          },
          "3759191272": {
            "progressionHash": 3759191272,
            "dailyProgress": 0,
            "dailyLimit": 0,
            "weeklyProgress": 0,
            "weeklyLimit": 0,
            "currentProgress": 0,
            "level": 1,
            "levelCap": 6,
            "stepIndex": 1,
            "progressToNextLevel": 0,
            "nextLevelAt": 1
          }
        },
        "clanCallsign": "HeM",
        "clanBannerData": {
          "decalId": 4142223388,
          "decalColorId": 3379387803,
          "decalBackgroundColorId": 3585526349,
          "gonfalonId": 1473910866,
          "gonfalonColorId": 2157636322,
          "gonfalonDetailId": 1698031298,
          "gonfalonDetailColorId": 4078567632
        }
      }
    },
    "founder": {
      "memberType": 5,
      "isOnline": false,
      "lastOnlineStatusChange": "1644181974",
      "groupId": "3074427",
      "destinyUserInfo": {
        "LastSeenDisplayName": "dec4234",
        "LastSeenDisplayNameType": 3,
        "iconPath": "/img/theme/bungienet/icons/steamLogo.png",
        "crossSaveOverride": 0,
        "applicableMembershipTypes": [
          3
        ],
        "isPublic": false,
        "membershipType": 3,
        "membershipId": "4611686018468620320",
        "displayName": "dec4234",
        "bungieGlobalDisplayName": "dec4234",
        "bungieGlobalDisplayNameCode": 9904
      },
      "bungieNetUserInfo": {
        "supplementalDisplayName": "dec4234#9904",
        "iconPath": "/img/profile/avatars/cc14.jpg",
        "crossSaveOverride": 0,
        "isPublic": false,
        "membershipType": 254,
        "membershipId": "17506516",
        "displayName": "dec4234",
        "bungieGlobalDisplayName": "dec4234",
        "bungieGlobalDisplayNameCode": 9904
      },
      "joinDate": "2019-12-02T22:23:19Z"
    },
    "alliedIds": [],
    "allianceStatus": 0,
    "groupJoinInviteCount": 0,
    "currentUserMembershipsInactiveForDestiny": false,
    "currentUserMemberMap": {},
    "currentUserPotentialMemberMap": {}
  },
  "ErrorCode": 1,
  "ThrottleSeconds": 0,
  "ErrorStatus": "Success",
  "Message": "Ok",
  "MessageData": {}
}
 */