use serde::{Deserialize, Deserializer};
use serde_json::Value;
use anyhow::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct BungieUser {
    #[serde(rename = "profiles")]
    pub memberships: Vec<DestinyProfile>,
    #[serde(skip)]
    pub primary: DestinyProfile,
    pub bnetMembership: BnetMembership,
}

impl BungieUser {
    pub fn new(response: &str) -> Result<Self> {
        let val: Value = serde_json::from_str(response)?;
        let list: Vec<DestinyProfile> = serde_json::from_value(val["Response"]["profiles"].clone())?;

        Ok(Self {
            memberships: list.clone(),
            primary: BungieUser::get_primary_profile(list).unwrap(),
            bnetMembership: serde_json::from_value::<BnetMembership>(val["Response"]["bnetMembership"].clone())?,
        })
    }

    /// Get the primary Profile associated with this account. A.k.a.
    /// the one that has taken precedence over other profiles connected
    /// to the account due to cross-save.
    fn get_primary_profile(list: Vec<DestinyProfile>) -> Option<DestinyProfile> {
        for info in list {
            if info.crossSaveOverride == info.platform || info.crossSaveOverride == 0 {
                return Some(info);
            }
        }

        None
    }
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
    pub platformDisplayName: String,
    pub crossSaveOverride: i16,
    #[serde(rename = "bungieGlobalDisplayName")]
    pub globalDisplayName: String,
    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub discriminator: i32,

    pub isPublic: bool,
    pub isOverridden: bool,
    pub isCrossSavePrimary: bool,

    #[serde(rename = "applicableMembershipTypes")]
    pub membershipTypes: Vec<i8>,
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
            platformDisplayName: "".to_string(),
            crossSaveOverride: 0,
            globalDisplayName: "".to_string(),
            discriminator: 0,
            isPublic: false,
            isOverridden: false,
            isCrossSavePrimary: false,
            membershipTypes: vec![]
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
            "isOverridden":false,
            "isCrossSavePrimary":false,
            "crossSaveOverride":0,
            "applicableMembershipTypes":[
               3
            ],
            "isPublic":false,
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
         "crossSaveOverride":0,
         "isPublic":false,
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