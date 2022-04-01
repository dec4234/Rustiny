use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::Util::date_deserializer_optional;
use anyhow::Result;
use crate::BungieUser;

#[derive(Deserialize, Serialize, Clone)]
pub struct DestinyCharacter {
    #[serde(skip)]
    pub user: Option<BungieUser>,

    pub baseCharacterLevel: i16,
    pub characterId: String,
    pub classHash: i64,
    pub classType: i16,
    #[serde(default = "date_deserializer_optional::default")]
    #[serde(with = "date_deserializer_optional")]
    pub dateLastPlayed: Option<NaiveDateTime>,
    pub emblemBackgroundPath: String,
    pub emblemColor: EmblemColor,
    pub emblemHash: i64,
    pub emblemPath: String,
    pub genderHash: i64,
    pub genderType: i16,
    pub levelProgression: LevelProgression,
    pub light: i16,
    pub membershipId: String,
    pub membershipType: i8,
    pub minutesPlayedThisSession: String,
    pub minutesPlayedTotal: String,
    pub percentToNextLevel: f32,
    pub raceHash: i64,
    pub raceType: i8,
    pub stats: CharacterStats,
    pub titleRecordHash: i64,
}

impl DestinyCharacter {
    pub fn new(value: Value) -> Result<Self> {
        Ok(serde_json::from_value::<DestinyCharacter>(value)?)
    }

    pub fn new_with_user(value: Value, user: BungieUser) -> Result<Self> {
        let mut char = serde_json::from_value::<DestinyCharacter>(value)?;
        char.user = Some(user);

        Ok(char)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EmblemColor {
    pub alpha: i16,
    pub blue: i16,
    pub green: i16,
    pub red: i16,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LevelProgression {
    pub currentProgress: i16,
    pub dailyLimit: i16,
    pub dailyProgress: i16,
    pub level: i16,
    pub levelCap: i16,
    pub nextLevelAt: i16,
    pub progressToNextLevel: i16,
    pub progressionHash: i64,
    pub stepIndex: i16,
    pub weeklyLimit: i16,
    pub weeklyProgress: i16,
}

/**
Stats about this character such as your
recovery, discipline, etc.

They are hashed in the response but renamed here
**/
#[derive(Deserialize, Serialize, Clone)]
pub struct CharacterStats {
    #[serde(rename = "144602215")]
    pub intellect: i16,
    #[serde(rename = "1735777505")]
    pub discipline: i16,
    #[serde(rename = "1935470627")]
    pub power: i16,
    #[serde(rename = "1943323491")]
    pub recovery: i16,
    #[serde(rename = "2996146975")]
    pub mobility: i16,
    #[serde(rename = "392767087")]
    pub resilience: i16,
    #[serde(rename = "4244567218")]
    pub strength: i16,
}