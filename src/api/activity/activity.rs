use tokio::sync::Mutex;
use crate::api::ApiClient::ApiClient;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::NaiveDateTime;
use serde_json::Value;
use crate::api::clan::Clan::DestinyUserInfo;
use crate::api::user::BungieUser::DestinyProfile;
use crate::api::Util::date_deserializer;
use crate::api::Util::macros;
use crate::basic_wrapped;
use crate::api::Util::macros::Basic;

pub struct PgcrScraper {
    client: ApiClient,
}

impl PgcrScraper {
    pub async fn new(client: &ApiClient) -> Self {
        Self {
            client: client.clone().await
        }
    }

    pub async fn get_pgcr(&self, id: i64) -> Result<PGCR> {
        Ok(PGCR::new(self.get_pgcr_raw(id).await?["Response"].clone())?)
    }

    pub async fn get_pgcr_raw(&self, id: i64) -> Result<Value> {
        let url = format!("https://stats.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activityId}/", activityId = id);
        let resp = self.client.get_parse::<Value>(url).await?;

        Ok(resp)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PGCR {
    #[serde(default = "date_deserializer::default")]
    #[serde(with = "date_deserializer")]
    pub period: Option<NaiveDateTime>,
    pub startingPhaseIndex: i8,
    pub activityWasStartedFromBeginning: bool,
    pub activityDetails: ActivityDetails,
    pub entries: Vec<Entry>,
}

impl PGCR {
    pub fn new(val: Value) -> Result<Self> {
        Ok(serde_json::from_value::<PGCR>(val)?)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ActivityDetails {
    pub referenceId: i64,
    pub directorActivityHash: i64,
    pub instanceId: String,
    pub mode: i8,
    pub modes: Vec<i16>,
    pub isPrivate: bool,
    pub membershipType: i8,
}

basic_wrapped!(Score);

#[derive(Deserialize, Serialize, Clone)]
pub struct Entry {
    pub standing: i16,
    pub score: Score,
    pub player: Player,
    pub characterId: String,
    pub values: Values,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Player {
    pub destinyUserInfo: DestinyProfile,
    pub characterClass: String,
    pub classHash: i64,
    pub raceHash: i64,
    pub genderHash: i64,
    pub characterLevel: i16,
    pub lightLevel: i16,
    pub emblemHash: i64,
}

basic_wrapped!(assists, completed, deaths, kills, opponentsDefeated, efficiency, killsDeathsRatio, killsDeathsAssists, activityDurationSeconds, completionReason);
basic_wrapped!(fireteamId, startSeconds, timePlayedSeconds, playerCount, teamScore);

#[derive(Deserialize, Serialize, Clone)]
pub struct Values {
    pub assists: assists,
    pub completed: completed,
    pub deaths: deaths,
    pub kills: kills,
    pub opponentsDefeated: opponentsDefeated,
    pub efficiency: efficiency,
    pub killsDeathsRatio: killsDeathsRatio,
    pub killsDeathsAssists: killsDeathsAssists,
    pub score: Score,
    pub activityDurationSeconds: activityDurationSeconds,
    pub completionReason: completionReason,
    pub fireteamId: fireteamId,
    pub startSeconds: startSeconds,
    pub timePlayedSeconds: timePlayedSeconds,
    pub playerCount: playerCount,
    pub teamScore: teamScore,
}