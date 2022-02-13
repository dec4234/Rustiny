use serde::Deserialize;

#[derive(Deserialize)]
pub struct BungieUser {

}

impl BungieUser {
    pub fn from_id(id: &str, platform: DestinyPlatform) -> Self {
        Self {

        }
    }
}

pub struct PlayerInfoCard {

}

pub enum DestinyPlatform {
    TigerXbox,
    TigerPSN,
    TigerSteam,
}

impl DestinyPlatform {
    pub fn get_code(&self) -> u8 {
        match self {
            DestinyPlatform::TigerXbox => 1,
            DestinyPlatform::TigerPSN => 2,
            DestinyPlatform::TigerSteam => 3,
        }
    }
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