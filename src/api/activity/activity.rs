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
use crate::basic;
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
        Ok(PGCR::new(self.get_pgcr_raw(id).await?)?)
    }

    pub async fn get_pgcr_raw(&self, id: i64) -> Result<Value> {
        let url = format!("https://stats.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activityId}/", activityId = id);
        let resp = self.client.get_parse::<Value>(url, true).await?;

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
    pub teams: Vec<Team>,
}

impl PGCR {
    pub fn new(val: Value) -> Result<Self> {
        Ok(serde_json::from_value::<PGCR>(val)?)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Team {
    pub teamId: i16,
    pub teamName: String,
    pub standing: standing,
    pub score: Score,
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

basic!(Score);

#[derive(Deserialize, Serialize, Clone)]
pub struct Entry {
    pub standing: i16,
    pub score: Score,
    pub player: Player,
    pub characterId: String,
    pub values: EntryValues,
    pub extended: Extended,
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

// Score defined above
basic!(assists, completed, deaths, kills, opponentsDefeated, efficiency, killsDeathsRatio, killsDeathsAssists, activityDurationSeconds, completionReason);
basic!(fireteamId, startSeconds, timePlayedSeconds, playerCount, teamScore);

// Optionally included
basic!(averageScorePerKill, averageScorePerLife, standing, team);

#[derive(Deserialize, Serialize, Clone)]
pub struct EntryValues {
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

    // Optional - PvP stuff
    pub averageScorePerKill: Option<averageScorePerKill>,
    pub averageScorePerLife: Option<averageScorePerLife>,
    pub standing: Option<standing>,
    pub team: Option<team>,

}

#[derive(Deserialize, Serialize, Clone)]
pub struct Extended {
    pub values: ExtendedValues,
    pub weapons: Option<Vec<WeaponData>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeaponData {
    pub referenceId: i64,
    pub values: WeaponDataValues,
}

basic!(uniqueWeaponKills, uniqueWeaponPrecisionKills, uniqueWeaponKillsPrecisionKills);

#[derive(Deserialize, Serialize, Clone)]
pub struct WeaponDataValues {
    pub uniqueWeaponKills: uniqueWeaponKills,
    pub uniqueWeaponPrecisionKills: uniqueWeaponPrecisionKills,
    pub uniqueWeaponKillsPrecisionKills: uniqueWeaponKillsPrecisionKills,
}

basic!(precisionKills, weaponKillsGrenade, weaponKillsMelee, weaponKillsSuper, weaponKillsAbility);
basic!(medalUnknown, allMedalsEarned);

#[derive(Deserialize, Serialize, Clone)]
pub struct ExtendedValues {

}

// Trials Of Osiris: 9496960718
// VotD: 10405562745