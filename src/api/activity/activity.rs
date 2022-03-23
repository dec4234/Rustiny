use tokio::sync::Mutex;
use crate::api::ApiClient::ApiClient;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::NaiveDateTime;
use serde_json::Value;
use strum::{EnumIter, IntoEnumIterator};
use crate::api::DestinyAPI::URL_BASE;
use crate::api::user::BungieUser::DestinyProfile;
use crate::api::Util::date_deserializer;
use crate::api::Util::macros;
use crate::{basic, BungieUser, DestinyCharacter};
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

    pub async fn clone(&self) -> Self {
        Self {
            client: self.client.clone().await
        }
    }

    pub async fn get_pgcr(&self, id: i64) -> Result<PGCR> {
        Ok(PGCR::new(self.get_pgcr_raw(id).await?)?)
    }

    /// Get this PGCR raw
    pub async fn get_pgcr_raw(&self, id: i64) -> Result<Value> {
        let url = format!("https://stats.bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/{activityId}/", activityId = id);
        let resp = self.client.get_parse::<Value>(url, true).await?;

        Ok(resp)
    }

    /// Get the activity history of this user of the specific activity
    pub async fn get_activity_history(&self, user: BungieUser, mode: ActivityMode) -> Result<Vec<ActivityHistoryResponse>> {
        let mut query = String::new();
        query.push_str(format!("?count=250&mode={mode}", mode = mode.get_code()).as_str());

        let mut vec = vec![];

        for chara in user.get_characters(&self.client).await? {
            'inner: for i in 0..1000 {
                let url = format!("{}/Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/Activities/{query}&page={page}", URL_BASE, membershipType = &user.primary.platform, destinyMembershipId = &user.primary.id, characterId = &chara.characterId, query = &query, page = i);

                let response = self.client.get_parse::<Value>(url, true).await?;
                let inner = serde_json::from_value::<Vec<ActivityHistoryResponse>>(response["activities"].clone());

                if let Ok(inner) = inner { // If there are no more valid pages this will catch it
                    for ahr in inner {
                        vec.push(ahr);
                    }
                } else {
                    break 'inner;
                }
            }
        }

        Ok(vec)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ActivityHistoryResponse {
    #[serde(default = "date_deserializer::default")]
    #[serde(with = "date_deserializer")]
    pub period: Option<NaiveDateTime>,
    pub activityDetails: ActivityDetails,
    pub values: EntryValues,
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
    pub precisionKills: precisionKills,
    pub weaponKillsGrenade: weaponKillsGrenade,
    pub weaponKillsMelee: weaponKillsMelee,
    pub weaponKillsSuper: weaponKillsSuper,
    pub weaponKillsAbility: weaponKillsAbility,

    pub medalUnknown: Option<medalUnknown>,
    pub allMedalsEarned: Option<allMedalsEarned>,
}

// Trials Of Osiris: 9496960718
// VotD: 10405562745

// https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-Definitions-DestinyActivityModeType.html#schema_Destiny-HistoricalStats-Definitions-DestinyActivityModeType
#[derive(EnumIter)]
pub enum ActivityMode {
    None,
    Story,
    Strike,
    Raid,
    AllPvP,
    Patrol,
    AllPvE,
    Control,
    Clash,
    CrimsonDoubles,
    Nightfall,
    HeroicNightfall,
    AllStrikes,

    IronBanner,
    AllMayhem,
    Supremacy,
    PrivateMatchesAll,
    Survival,
    Countdown,
    TrialsOfTheNine,
    Social,
    TrialsCountdown,
    TrialsSurvival,
    IronBannerControl,
    IronBannerClash,
    IronBannerSupremacy,
    ScoredNightfall,
    ScoredHeroicNightfall,
    Rumble,
    AllDoubles,
    Doubles,
    PrivateMatchesClash,
    PrivateMatchesControl,
    PrivateMatchesSupremacy,
    PrivateMatchesCountdown,
    PrivateMatchesSurvival,
    PrivateMatchesMayhem,
    PrivateMatchesRumble,
    HeroicAdventure,
    Showdown,
    Lockdown,
    Scorched,
    ScorchedTeam,
    Gambit,
    AllPvECompetitve,

    Breakthrough,
    BlackArmoryRun,
    Salvage,
    IronBannerSalvage,
    PvPCompetitve,
    PvPQuickplay,
    ClashQuickplay,
    ClashCompetitve,
    ControlQuickplay,
    ControlCompetitve,
    GambitPrime,
    Reckoning,
    Menagerie,
    VexOffensive,
    NightmareHunt,
    Elimination,
    Momentum,
    Dungeon,
    Sundial,
    TrialsOfOsiris,
    Dares,
    Offensive,
}

impl ActivityMode {
    pub fn get_code(&self) -> i16 {
        match self {
            ActivityMode::None => {0}
            ActivityMode::Story => {2}
            ActivityMode::Strike => {3}
            ActivityMode::Raid => {4}
            ActivityMode::AllPvP => {5}
            ActivityMode::Patrol => {6}
            ActivityMode::AllPvE => {7}
            ActivityMode::Control => {10}
            ActivityMode::Clash => {12}
            ActivityMode::CrimsonDoubles => {15}
            ActivityMode::Nightfall => {16}
            ActivityMode::HeroicNightfall => {17}
            ActivityMode::AllStrikes => {18}
            ActivityMode::IronBanner => {19}
            ActivityMode::AllMayhem => {25}
            ActivityMode::Supremacy => {31}
            ActivityMode::PrivateMatchesAll => {32}
            ActivityMode::Survival => {37}
            ActivityMode::Countdown => {38}
            ActivityMode::TrialsOfTheNine => {39}
            ActivityMode::Social => {40}
            ActivityMode::TrialsCountdown => {41}
            ActivityMode::TrialsSurvival => {42}
            ActivityMode::IronBannerControl => {43}
            ActivityMode::IronBannerClash => {44}
            ActivityMode::IronBannerSupremacy => {45}
            ActivityMode::ScoredNightfall => {46}
            ActivityMode::ScoredHeroicNightfall => {47}
            ActivityMode::Rumble => {48}
            ActivityMode::AllDoubles => {49}
            ActivityMode::Doubles => {50}
            ActivityMode::PrivateMatchesClash => {51}
            ActivityMode::PrivateMatchesControl => {52}
            ActivityMode::PrivateMatchesSupremacy => {53}
            ActivityMode::PrivateMatchesCountdown => {54}
            ActivityMode::PrivateMatchesSurvival => {55}
            ActivityMode::PrivateMatchesMayhem => {56}
            ActivityMode::PrivateMatchesRumble => {57}
            ActivityMode::HeroicAdventure => {58}
            ActivityMode::Showdown => {59}
            ActivityMode::Lockdown => {60}
            ActivityMode::Scorched => {61}
            ActivityMode::ScorchedTeam => {62}
            ActivityMode::Gambit => {63}
            ActivityMode::AllPvECompetitve => {64}
            ActivityMode::Breakthrough => {65}
            ActivityMode::BlackArmoryRun => {66}
            ActivityMode::Salvage => {67}
            ActivityMode::IronBannerSalvage => {68}
            ActivityMode::PvPCompetitve => {69}
            ActivityMode::PvPQuickplay => {70}
            ActivityMode::ClashQuickplay => {71}
            ActivityMode::ClashCompetitve => {72}
            ActivityMode::ControlQuickplay => {73}
            ActivityMode::ControlCompetitve => {74}
            ActivityMode::GambitPrime => {75}
            ActivityMode::Reckoning => {76}
            ActivityMode::Menagerie => {77}
            ActivityMode::VexOffensive => {78}
            ActivityMode::NightmareHunt => {79}
            ActivityMode::Elimination => {80}
            ActivityMode::Momentum => {81}
            ActivityMode::Dungeon => {82}
            ActivityMode::Sundial => {83}
            ActivityMode::TrialsOfOsiris => {84}
            ActivityMode::Dares => {85}
            ActivityMode::Offensive => {86}
        }
    }

    pub fn from_code(code: i16) -> Option<ActivityMode> {
        for mode in ActivityMode::iter() {
            if mode.get_code() == code {
                return Some(mode);
            }
        }

        None
    }
}