use serde::{Deserialize};
use serde_json::{json, Value};
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use crate::api::ApiClient::ApiClient;
use crate::api::DestinyAPI;
use crate::api::Util::date_deserializer;

#[derive(Deserialize, Debug, Clone)]
pub struct BungieUser {
    #[serde(rename = "profiles")]
    pub memberships: Vec<DestinyProfile>,
    #[serde(skip)]
    pub primary: DestinyProfile,
    #[serde(rename = "bnetMembership")]
    pub bnet_membership: BnetMembership,
}

impl BungieUser {
    pub fn new(val: Value) -> Result<Self> {
        // let val: Value = serde_json::from_str(response)?;
        let list: Vec<DestinyProfile> = serde_json::from_value(val["Response"]["profiles"].clone())?;

        Ok(Self {
            memberships: list.clone(),
            primary: BungieUser::get_primary_profile(list).unwrap(),
            bnet_membership: serde_json::from_value::<BnetMembership>(val["Response"]["bnetMembership"].clone())?,
        })
    }

    pub async fn get_user_by_id(client: &ApiClient, id: &str, platform: DestinyPlatform) -> Result<BungieUser> {
        let url = format!("{}/Destiny2/{membershipType}/Profile/{membershipId}/LinkedProfiles/", DestinyAPI::URL_BASE, membershipId = id, membershipType = platform.get_code());
        let val = serde_json::from_str::<Value>(client.get(url).await?.as_str())?;
        BungieUser::new(val.clone())
    }

    pub async fn get_user_by_name_and_discrim_with_platform(client: &ApiClient, name_and_discrim: String, platform: DestinyPlatform) -> Result<BungieUser> {
        let url = format!("{}/Destiny2/SearchDestinyPlayerByBungieName/{membershipType}/", DestinyAPI::URL_BASE, membershipType = platform.get_code());
        let split: Vec<&str> = name_and_discrim.split("#").collect();

        if split.len() != 2 {
            return Err(anyhow!("The name of the user, when split at the # did not result in 2 components. Are you sure you passed a name and discriminator such as dec4234#9904 ?"));
        }

        let body = json!({
            "displayName": split[0],
            "displayNameCode": split[1],
        });

        let val = client.post_parse::<Value>(url, body.to_string()).await?;
        let list = serde_json::from_value::<Vec<PartialProfileResponse>>(val["Response"].clone())?;

        for profile in list {
            return BungieUser::get_user_by_id(client, profile.membershipId.as_str(), DestinyPlatform::from_code(profile.membershipType).expect("Platform Code could not be deserialized")).await;
        }

        Err(anyhow!("Returned List was Empty, check your search query"))
    }

    /// Get the primary Profile associated with this account. A.k.a.
    /// the one that has taken precedence over other profiles connected
    /// to the account due to cross-save.
    fn get_primary_profile(list: Vec<DestinyProfile>) -> Option<DestinyProfile> {
        for info in list {
            if info.cross_save_override == info.platform || info.cross_save_override == 0 {
                return Some(info);
            }
        }

        None
    }
}

#[derive(Deserialize)]
struct PartialProfileResponse {
    pub membershipId: String,
    pub membershipType: i16,
}

/// A Destiny Profile, pertaining to an account on
/// a specific platform. Due to cross-save, a user
/// can have multiple profiles on their account.
#[derive(Deserialize, Debug, Clone)]
pub struct DestinyProfile {
    #[serde(rename = "membershipId")]
    pub id: String,
    #[serde(rename = "membershipType")]
    pub platform: i16,

    #[serde(rename = "displayName")]
    pub platform_display_name: String,
    #[serde(rename = "crossSaveOverride")]
    pub cross_save_override: i16,
    #[serde(rename = "bungieGlobalDisplayName")]
    pub global_display_name: String,
    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub discriminator: i32,

    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "isOverridden")]
    pub is_overridden: bool,
    #[serde(rename = "isCrossSavePrimary")]
    pub is_cross_save_primary: bool,

    #[serde(rename = "applicableMembershipTypes")]
    pub membership_types: Vec<i8>,

    #[serde(with = "date_deserializer")]
    pub dateLastPlayed: NaiveDateTime,
}

impl DestinyProfile {
    pub fn get_platform(&self) -> Option<DestinyPlatform> {
        DestinyPlatform::from_code(self.platform)
    }
}

impl Default for DestinyProfile {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            platform: 0,
            platform_display_name: "".to_string(),
            cross_save_override: 0,
            global_display_name: "".to_string(),
            discriminator: 0,
            is_public: false,
            is_overridden: false,
            is_cross_save_primary: false,
            membership_types: vec![],
            dateLastPlayed: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

pub enum DestinyPlatform {
    None,
    Xbox,
    PSN,
    Steam,
    Blizzard,
    Stadia,
    Demon,
    BungieNext,
    All,
}

impl DestinyPlatform {
    pub fn from_code(code: i16) -> Option<Self> {
        match code {
            0 => Some(DestinyPlatform::None),
            1 => Some(DestinyPlatform::Xbox),
            2 => Some(DestinyPlatform::PSN),
            3 => Some(DestinyPlatform::Steam),
            4 => Some(DestinyPlatform::Blizzard),
            5 => Some(DestinyPlatform::Stadia),
            10 => Some(DestinyPlatform::Demon),
            254 => Some(DestinyPlatform::BungieNext),
            -1 => Some(DestinyPlatform::All),
            _ => None
        }
    }

    pub fn get_code(&self) -> i16 {
        match self {
            DestinyPlatform::None => 0,
            DestinyPlatform::Xbox => 1,
            DestinyPlatform::PSN => 2,
            DestinyPlatform::Steam => 3,
            DestinyPlatform::Blizzard => 4,
            DestinyPlatform::Stadia => 5,
            DestinyPlatform::Demon => 10,
            DestinyPlatform::BungieNext => 254,
            DestinyPlatform::All => -1,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BnetMembership {
    #[serde(rename = "supplementalDisplayName")]
    pub combined_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "membershipId")]
    pub bnet_membership_id: String,
}

/*
{
   "Response":{
      "profiles":[
         {
            "dateLastPlayed":"2022-02-06T21:12:49Z",
            "is_overridden":false,
            "is_cross_save_primary":false,
            "cross_save_override":0,
            "applicableMembershipTypes":[
               3
            ],
            "is_public":false,
            "membershipType":3,
            "membershipId":"4611686018468620320",
            "displayName":"dec4234",
            "bungieGlobalDisplayName":"dec4234",
            "bungieGlobalDisplayNameCode":9904
         }
      ],
      "bnetMembership":{
         "supplementalDisplayName":"dec4234#9904",
         "iconPath":"/img/profile/avatars/cc14.jpg",
         "cross_save_override":0,
         "is_public":false,
         "membershipType":254,
         "membershipId":"17506516",
         "displayName":"dec4234",
         "bungieGlobalDisplayName":"dec4234",
         "bungieGlobalDisplayNameCode":9904
      },
      "profilesWithErrors":[

      ]
   },
   "ErrorCode":1,
   "ThrottleSeconds":0,
   "ErrorStatus":"Success",
   "Message":"Ok",
   "MessageData":{

   }
}

 */