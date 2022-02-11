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