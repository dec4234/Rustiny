use serde::{Deserialize, Deserializer};
use serde_json::Value;
use anyhow::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct BungieUser {
    #[serde(rename = "profiles")]
    pub memberships: Vec<DestinyMembershipInfo>,
    #[serde(skip)]
    pub primary: DestinyMembershipInfo,
}

impl BungieUser {
    pub fn new(response: &str) -> Result<Self> {
        let val: Value = serde_json::from_str(response)?;
        let list: Vec<DestinyMembershipInfo> = serde_json::from_value(val["Response"]["profiles"].clone())?;

        Ok(Self {
            memberships: list.clone(),
            primary: BungieUser::get_primary_membership(list).unwrap(),
        })
    }

    fn get_primary_membership(list: Vec<DestinyMembershipInfo>) -> Option<DestinyMembershipInfo> {
        for info in list {
            return Some(info);
        }

        None
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DestinyMembershipInfo {
    #[serde(rename = "membershipId")]
    pub id: String,
    #[serde(rename = "membershipType")]
    pub platform: i16,

    #[serde(rename = "displayName")]
    pub platformDisplayName: String,

    pub isPublic: bool,
    pub isOverridden: bool,
    pub isCrossSavePrimary: bool,
}

impl DestinyMembershipInfo {
    pub fn get_platform(&self) -> Option<DestinyPlatform> {
        DestinyPlatform::from_code(self.platform)
    }
}

impl Default for DestinyMembershipInfo {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            platform: 0,
            platformDisplayName: "".to_string(),
            isPublic: false,
            isOverridden: false,
            isCrossSavePrimary: false
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

#[derive(Deserialize)]
pub struct BnetMembership {
    #[serde(rename = "supplementalDisplayName")]
    full_name: String,

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