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
use crate::{basic, BungieUser, DestinyCharacter, enumize};
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
        query.push_str(format!("?count=250&mode={mode}", mode = mode.get()).as_str());

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
    #[serde(default)]
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
enumize!(ActivityMode, i16 => {
    None, 0,
    Story, 2,
    Strike, 3,
    Raid, 4,
    AllPvP, 5,
    Patrol, 6,
    AllPvE, 7,
    Control, 10,
    Clash, 12,
    CrimsonDoubles, 15,
    Nightfall, 16,
    HeroicNightfall, 17,
    AllStrikes, 18,
    IronBanner, 19,
    AllMayhem, 25,
    Supremacy, 31,
    PrivateMatchesAll, 32,
    Survival, 37,
    Countdown, 38,
    TrialsOfTheNine, 39,
    Social, 40,
    TrialsCountdown, 41,
    TrialsSurvival, 42,
    IronBannerControl, 43,
    IronBannerClash, 44,
    IronBannerSupremacy, 45,
    ScoredNightfall, 46,
    ScoredHeroicNightfall, 47,
    Rumble, 48,
    AllDoubles, 49,
    Doubles, 50,
    PrivateMatchesClash, 51,
    APrivateMatchesControl, 52,
    PrivateMatchesSupremacy, 53,
    PrivateMatchesCountdown, 54,
    PrivateMatchesSurvival, 55,
    PrivateMatchesMayhem, 56,
    PrivateMatchesRumble, 57,
    HeroicAdventure, 58,
    Showdown, 59,
    Lockdown, 60,
    Scorched, 61,
    ScorchedTeam, 62,
    Gambit, 63,
    AllPvECompetitve, 64,
    Breakthrough, 65,
    BlackArmoryRun, 66,
    Salvage, 67,
    IronBannerSalvage, 68,
    PvPCompetitve, 69,
    PvPQuickplay, 70,
    ClashQuickplay, 71,
    ClashCompetitve, 72,
    ControlQuickplay, 73,
    ControlCompetitve, 74,
    GambitPrime, 75,
    Reckoning, 76,
    Menagerie, 77,
    VexOffensive, 78,
    NightmareHunt, 79,
    Elimination, 80,
    Momentum, 81,
    Dungeon, 82,
    Sundial, 83,
    TrialsOfOsiris, 84,
    Dares, 85,
    Offensive, 86
});

// https://github.com/dec4234/JavaDestinyAPI/blob/master/src/main/java/net/dec4234/javadestinyapi/stats/activities/ActivityIdentifier.java
enumize!(ActivityIdentifier, (ActivityMode, Vec<String>) => {
    // Strikes
    ArmsDealer, (ActivityMode::Strike, vec!["442671778".to_string(), "2080275457".to_string(), "2378719026".to_string(), "2724706103".to_string(), "2378719025".to_string(), "770196931".to_string(), "3240321863".to_string(), "1258914202".to_string(), "1679518121".to_string()]),
    LakeOfShadows, (ActivityMode::Strike, vec!["2318521576".to_string(), "3711627564".to_string(), "3725993747".to_string(), "2630091891".to_string(), "4134816102".to_string()]),

    TheDisgraced, (ActivityMode::Strike, vec!["1684420962".to_string(), "174131855".to_string()]),
    FallenSaber, (ActivityMode::Strike, vec!["3597990372".to_string(), "3777220691".to_string()]),
    DevilsLair, (ActivityMode::Strike, vec!["969982762".to_string()]),

    SavathunsSong, (ActivityMode::Strike, vec!["2359594803".to_string(), "1101792305".to_string(), "3191123858".to_string(), "649648599".to_string(), "1542611209".to_string()]),

    InvertedSpire, (ActivityMode::Strike, vec!["3704910925".to_string(), "1563393783".to_string(), "286562305".to_string(), "1107473294".to_string(), "1743518003".to_string(), "338662534".to_string(), "2753180142".to_string(), "1743518000".to_string(), "467266668".to_string()]),
    ExodusCrash, (ActivityMode::Strike, vec!["2459768558".to_string(), "1549614516".to_string(), "4260306233".to_string(), "1930116823".to_string(), "2479262829".to_string(), "1930116820".to_string(), "2971335647".to_string()]),
    InsightTerminus, (ActivityMode::Strike, vec!["3751421841".to_string(), "291911094".to_string(), "3735153516".to_string(), "3735153519".to_string()]),
    ProvingGround, (ActivityMode::Strike, vec!["546528643".to_string(), "1754609040".to_string()]),

    ThePyramidion, (ActivityMode::Strike, vec!["1035135049".to_string(), "1603374112".to_string(), "1332567112".to_string(), "2704613535".to_string(), "1332567115".to_string(), "981383202".to_string(), "2799837309".to_string(), "4261351281".to_string()]),
    FesteringCore, (ActivityMode::Strike, vec!["1035850837".to_string(), "3596828104".to_string()]),

    TreeOfProbabilities, (ActivityMode::Strike, vec!["2678510381".to_string(), "1263901594".to_string(), "561345572".to_string(), "561345575".to_string(), "840678113".to_string(), "4085493024".to_string(), "2684121894".to_string()]),
    AGardenWorld, (ActivityMode::Strike, vec!["656703508".to_string(), "3676029623".to_string(), "2230236215".to_string(), "2230236212".to_string(), "689927878".to_string(), "117447065".to_string(), "2579344189".to_string(), "743963294".to_string()]),

    StrangeTerrain, (ActivityMode::Strike, vec!["2992505404".to_string(), "861639649".to_string(), "3801775390".to_string(), "2248296964".to_string(), "861639650".to_string()]),
    WillOfTheThousands, (ActivityMode::Strike, vec!["1198216109".to_string(), "3944547192".to_string(), "3510043585".to_string(), "1317492847".to_string(), "1891220709".to_string(), "3944547195".to_string()]),

    WardenOfNothing, (ActivityMode::Strike, vec!["1360385764".to_string(), "1360385767".to_string(), "1134446996".to_string(), "1493405720".to_string()]),
    TheHollowedLair, (ActivityMode::Strike, vec!["663301842".to_string(), "1475539136".to_string(), "1475539139".to_string(), "955874134".to_string()]),
    Broodhold, (ActivityMode::Strike, vec!["1666283939".to_string(), "3813623455".to_string()])
});

impl ActivityIdentifier {
    pub fn from_identifier(id: String) -> Option<ActivityIdentifier> {
        for ai in ActivityIdentifier::get_all() {
            for s in ai.get().1 {
                if s == id {
                    return Some(ai);
                }
            }
        }

        None
    }
}