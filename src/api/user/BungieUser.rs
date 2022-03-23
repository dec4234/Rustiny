use std::fmt::format;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use crate::api::ApiClient::ApiClient;
use crate::api::DestinyAPI;
use crate::api::DestinyAPI::URL_BASE;
use crate::api::Util::date_deserializer;
use crate::api::user::DestinyCharacter::DestinyCharacter;

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

    pub async fn get_user_by_id(client: &ApiClient, id: String, platform: DestinyPlatform) -> Result<BungieUser> {
        let url = format!("{}/Destiny2/{membershipType}/Profile/{membershipId}/LinkedProfiles/", DestinyAPI::URL_BASE, membershipId = id, membershipType = platform.get_code());
        let val = serde_json::from_str::<Value>(client.get(url).await?.as_str())?;
        BungieUser::new(val)
    }

    pub async fn get_users_with_name(client: &ApiClient, name: String) -> Result<Vec<DestinyProfile>> {
        let body = json!({
            "displayNamePrefix": name,
        });
        let mut list: Vec<DestinyProfile> = vec![];
        let mut count = 0;

        loop {
            let url = format!("{base}/User/Search/GlobalName/{page}/", base = URL_BASE, page = count);

            let response = client.post(url, body.to_string()).await?;
            let val = serde_json::from_str::<Value>(response.as_str())?["Response"].clone();

                if let Some(map) = val.as_object() {
                    if let Some(newVal) = val["searchResults"].clone().as_array() {
                        for v in newVal {
                            let profiles = serde_json::from_value::<Vec<DestinyProfile>>(v["destinyMemberships"].clone());

                            if let Ok(profiles) = profiles {
                                if !profiles.is_empty() {
                                    if let Some(user) = BungieUser::get_primary_profile(profiles) {
                                        list.push(user);
                                    }
                                }
                            } else {
                                return Err(anyhow!("Something went wrong when deserializing list of profiles!"));
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }

            count += 1;
        }

        Ok(list)
    }

    pub async fn get_user_by_name_and_discrim_with_platform(client: &ApiClient, name_and_discrim: String, platform: DestinyPlatform) -> Result<BungieUser> {
        let url = format!("{}/Destiny2/SearchDestinyPlayerByBungieName/{membershipType}/", DestinyAPI::URL_BASE, membershipType = platform.get_code());
        let split: Vec<&str> = name_and_discrim.split("#").collect();

        if split.len() != 2 {
            return Err(anyhow!("{} - The name of the user, when split at the # did not result in 2 components. Are you sure you passed a name and discriminator such as dec4234#9904 ?", name_and_discrim));
        }

        let body = json!({
            "displayName": split[0],
            "displayNameCode": split[1],
        });

        let list = client.post_parse::<Vec<PartialProfileResponse>>(url, body.to_string(), true).await?;

        if let Some(profile) = list.into_iter().next() {
            return BungieUser::get_user_by_id(client, profile.membershipId, DestinyPlatform::from_code(profile.membershipType).expect("Platform Code could not be deserialized")).await;
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

    fn get_primary_partial_profile(list: Vec<PartialProfileResponse>) -> Option<PartialProfileResponse> {
        for info in list {
            if info.crossSaveOverride == info.membershipType || info.crossSaveOverride == 0 {
                return Some(info);
            }
        }

        None
    }

    pub async fn get_characters(&self, client: &ApiClient) -> Result<Vec<DestinyCharacter>> {
        let mut vec = vec![];

        let resp = client.get(format!("{}/Destiny2/{membershipType}/Profile/{destinyMembershipId}/?components=Characters", URL_BASE, membershipType = self.primary.platform, destinyMembershipId = self.primary.id)).await?;

        let val = serde_json::from_str::<Value>(resp.as_str())?["Response"]["characters"]["data"].clone();

        if let Some(map) = val.as_object() {
            for (k, v) in map {
                vec.push(DestinyCharacter::new(v.clone())?)
            }
        }

        Ok(vec)
    }
}

#[derive(Deserialize)]
struct PartialProfileResponse {
    pub membershipId: String,
    pub membershipType: i16,
    pub crossSaveOverride: i16,
}

/// A Destiny Profile, pertaining to an account on
/// a specific platform. Due to cross-save, a user
/// can have multiple profiles on their account.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DestinyProfile {
    #[serde(rename = "membershipId")]
    pub id: String,
    #[serde(rename = "membershipType")]
    pub platform: i16,

    #[serde(default)]
    #[serde(rename = "displayName")]
    pub platform_display_name: String,
    #[serde(rename = "crossSaveOverride")]
    pub cross_save_override: i16,

    #[serde(default)]
    #[serde(rename = "bungieGlobalDisplayName")]
    pub global_display_name: String,
    #[serde(default)]
    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub discriminator: i32,

    #[serde(rename = "isPublic")]
    pub is_public: bool,

    #[serde(rename = "isOverridden")]
    pub is_overridden: Option<bool>,

    #[serde(rename = "isCrossSavePrimary")]
    pub is_cross_save_primary: Option<bool>,

    #[serde(default)]
    #[serde(rename = "applicableMembershipTypes")]
    pub membership_types: Vec<i8>,

    #[serde(default = "date_deserializer::default")]
    #[serde(with = "date_deserializer")]
    pub dateLastPlayed: Option<NaiveDateTime>,

    // Only present in Clan Member reponses and Founder profile
    pub LastSeenDisplayName: Option<String>,
    pub LastSeenDisplayNameType: Option<i16>,
}

impl DestinyProfile {
    pub fn get_platform(&self) -> Option<DestinyPlatform> {
        DestinyPlatform::from_code(self.platform)
    }

    pub async fn get_bungie_user(&self, client: &ApiClient) -> Result<BungieUser> {
        BungieUser::get_user_by_id(client, self.id.clone(), DestinyPlatform::from_code(self.platform).unwrap()).await
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
            is_overridden: None,
            is_cross_save_primary: None,
            membership_types: vec![],
            dateLastPlayed: None,
            LastSeenDisplayName: None,
            LastSeenDisplayNameType: None
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

#[derive(Deserialize, Serialize, Debug, Clone)]
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