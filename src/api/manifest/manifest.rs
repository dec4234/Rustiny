use std::fs;
use crate::api::ApiClient::ApiClient;
use anyhow::Result;
use serde_json::Value;
use crate::api::DestinyAPI::URL_BASE;
use serde::{Deserialize, Serialize};
use sqlite::Connection;
use crate::enumize;

pub struct Manifest {
    client: ApiClient,
}

impl Manifest {
    pub fn new(client: ApiClient) -> Self {
        Self {
            client,
        }
    }

    pub async fn manifest(&self, typ: ManifestEntityType, hash: i64) -> Result<String> {
        let resp = self.client.get(format!("{}/Destiny2/Manifest/{entityType}/{hashIdentifier}/", URL_BASE, entityType = typ.get(), hashIdentifier = hash)).await?;

        Ok(resp)
    }

    pub async fn manifest_get(&self, typ: ManifestEntityType, hash: String) -> Result<String> {
        let resp = self.client.get(format!("{}/Destiny2/Manifest/{entityType}/{hashIdentifier}/", URL_BASE, entityType = typ.get(), hashIdentifier = hash)).await?;

        Ok(resp)
    }

    pub async fn get_manifest_info(&self) -> Result<ManifestInfoResponse> {
        self.client.get_parse::<ManifestInfoResponse>(format!("{}/Destiny2/Manifest/", URL_BASE), true).await
    }

    pub async fn manifest_reward(&self, milestoneHash: i64, rewardEntryHash: i64) -> Result<RewardInfo> {
        let resp = serde_json::from_str::<Value>(self.manifest(ManifestEntityType::MILESTONE, milestoneHash).await?.as_str())?;

        let resp = resp["Response"]["rewards"].clone();
        let resp = serde_json::from_value::<Rewards>(resp)?;

        let inner = match rewardEntryHash {
            // Current Week
            3789021730 => { resp.currentWeek.rewardEntries.nightfall.unwrap() },
            248695599 => { resp.currentWeek.rewardEntries.gambit.unwrap() },
            2043403989 => { resp.currentWeek.rewardEntries.raid.unwrap() },
            964120289 => { resp.currentWeek.rewardEntries.pvp.unwrap() }

            // Previous week
            305996677 => { resp.previousWeek.rewardEntries.PWnightfall.unwrap() },
            1514402550 => { resp.previousWeek.rewardEntries.PWgambit.unwrap() },
            783563440 => { resp.previousWeek.rewardEntries.PWraid.unwrap() },
            1478801436 => { resp.previousWeek.rewardEntries.PWpvp.unwrap() },
            _ => {
                panic!("Unknown ID Of Clan Weekly Reward Info");
            }
        };

        Ok(inner)
    }
}

/// This is where the Local Manifest Manager will live in the future
/// 
/// Non-functional at the moment, use Manifest::manifest()
pub struct LocalManifest {
    client: ApiClient,
    connection: Connection,
}

impl LocalManifest {
    pub async fn load(client: &ApiClient, path: String, version: String) -> Result<Self> {
        let info = Manifest::new(client.clone().await).get_manifest_info().await?;

        let connection = Connection::open(&path)?;

        Ok(Self {
            client: client.clone().await,
            connection,
        })
    }


}

#[derive(Deserialize, Serialize, Clone)]
pub struct ManifestInfoResponse {
    pub version: String,
    pub mobileAssetContentPath: String,
    pub mobileWorldContentPaths: MobileWorldContentPaths,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MobileWorldContentPaths {
    pub en: String,
    pub fr: String,
    pub es: String,
    #[serde(rename = "es-mx")]
    pub esmx: String,
    pub de: String,
    pub it: String,
    pub ja: String,
    #[serde(rename = "pt-br")]
    pub ptbr: String,
    pub ru: String,
    pub pl: String,
    pub ko: String,
    #[serde(rename = "zh-cht")]
    pub zhcht: String,
    #[serde(rename = "zh-chs")]
    pub zhchs: String,
}

enumize!(ManifestLanguage, String => {
    English, "en".to_string(),
    French, "fr".to_string(),
    Espanol, "es".to_string(),
    EspanolMexico, "es-mx".to_string(),
    Deutsch, "de".to_string(),
    Italian, "it".to_string(),
    Japanese, "ja".to_string(),
    PortugueseBrazil, "pt-br".to_string(),
    Russian, "ru".to_string(),
    Polish, "pl".to_string(),
    Korean, "ko".to_string(),
    ChineseTraditional, "zh-cht".to_string(),
    ChineseSimplified, "zh-chs".to_string()
});

#[derive(Deserialize, Serialize, Clone)]
pub struct Rewards {
    #[serde(rename = "1064137897")]
    pub currentWeek: RewardGroup,
    #[serde(rename = "4258746474")]
    pub previousWeek: RewardGroup,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RewardGroup {
    pub categoryHash: i64,
    pub categoryIdentifier: String,
    pub displayProperties: DisplayProperties,
    pub rewardEntries: RewardEntries,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RewardEntries {
    #[serde(rename = "3789021730")]
    pub nightfall: Option<RewardInfo>,
    #[serde(rename = "248695599")]
    pub gambit: Option<RewardInfo>,
    #[serde(rename = "2043403989")]
    pub raid: Option<RewardInfo>,
    #[serde(rename = "964120289")]
    pub pvp: Option<RewardInfo>,

    // Previous week
    #[serde(rename = "305996677")]
    pub PWnightfall: Option<RewardInfo>,
    #[serde(rename = "1514402550")]
    pub PWgambit: Option<RewardInfo>,
    #[serde(rename = "783563440")]
    pub PWraid: Option<RewardInfo>,
    #[serde(rename = "1478801436")]
    pub PWpvp: Option<RewardInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RewardInfo {
    pub rewardEntryHash: i64,
    pub rewardEntryIdentifier: String,
    pub items: Vec<RewardItem>,
    pub vendorHash: i64,
    pub displayProperties: DisplayProperties,
    pub order: i32,
    pub earnedUnlockHash: i64,
    pub redeemedUnlockHash: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RewardItem {
    pub itemHash: i64,
    pub quantity: i32,
    pub hasConditionalVisibility: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DisplayProperties {
    pub description: String,
    pub name: String,
    pub icon: Option<String>,
    pub hasIcon: bool,
}

enumize!(ManifestEntityType, String => {
    ARTIFACT, "DestinyArtifactDefinition".to_string(),
    BREAKERTYPE, "DestinyBreakerTypeDefinition".to_string(),
    COLLECTIBLE, "DestinyCollectibleDefinition".to_string(),
    EQUIPMENTSLOT, "DestinyEquipmentSlotDefinition".to_string(),
    INVENTORYBUCKET, "DestinyInventoryBucketDefinition".to_string(),

    INVENTORYITEM, "DestinyInventoryItemDefinition".to_string(),
    ITEMCATEGORY, "DestinyItemCategoryDefinition".to_string(),
    ITEMSTAT, "DestinyStatDefinition".to_string(),
    ITEMSTATGROUP, "DestinyStatGroupDefinition".to_string(),
    ITEMTIER, "DestinyItemTierTypeDefinition".to_string(),
    MATERIALREQUIREMENTSET, "DestinyMaterialRequirementSetDefinition".to_string(),
    POWERCAP, "DestinyPowerCapDefinition".to_string(),
    RECORD, "DestinyRecordDefinition".to_string(),
    REWARDSOURCE, "DestinyRewardSourceDefinition".to_string(),
    SANDBOXPERK, "DestinySandboxPerkDefinition".to_string(),
    TALENTGRID, "DestinyTalentGridDefinition".to_string(),
    REWARDENTRY, "DestinyMilestoneRewardEntryDefinition".to_string(),

    CLASS, "DestinyClassDefinition".to_string(),
    GENDER, "DestinyGenderDefinition".to_string(),
    MILESTONE, "DestinyMilestoneDefinition".to_string(),
    PROGRESSION, "DestinyProgressionDefinition".to_string(),
    RACE, "DestinyRaceDefinition".to_string(),

    ACTIVITY, "DestinyActivityDefinition".to_string(),
    ACTIVITYGRAPH, "DestinyActivityGraphDefinition".to_string(),
    ACTIVITYMODE, "DestinyActivityModeDefinition".to_string(),
    ACTIVITYMODIFIER, "DestinyActivityModifierDefinition".to_string(),
    ACTIVITYTYPE, "DestinyActivityTypeDefinition".to_string(),

    DAMAGETYPE, "DestinyDamageTypeDefinition".to_string(),
    DESTINATION, "DestinyDestinationDefinition".to_string(),
    FACTION, "DestinyFactionDefinition".to_string(),
    LOCATION, "DestinyLocationDefinition".to_string(),
    OBJECTIVE, "DestinyObjectiveDefinition".to_string(),
    PLACE, "DestinyPlaceDefinition".to_string(),
    VENDOR, "DestinyVendorDefinition".to_string(),
    VENDORGROUP, "DestinyVendorGroupDefinition".to_string(),

    CHECKLIST, "DestinyChecklistDefinition".to_string(),
    ENERGYTYPE, "DestinyEnergyTypeDefinition".to_string(),
    HISTORICALSTATS, "DestinyHistoricalStatsDefinition".to_string(),
    PRESENTATIONNODE, "DestinyPresentationNodeDefinition".to_string(),
    LORE, "DestinyLoreDefinition".to_string(),
    METRIC, "DestinyMetricDefinition".to_string(),
    PLUGSET, "DestinyPlugSetDefinition".to_string(),
    REPORTREASONCATEGORY, "DestinyReportReasonCategoryDefinition".to_string(),
    SEASON, "DestinySeasonDefinition".to_string(),
    SEASONPASS, "DestinySeasonPassDefinition".to_string(),
    SOCKETCATEGORY, "DestinySocketCategoryDefinition".to_string(),
    SOCKETTYPE, "DestinySocketTypeDefinition".to_string(),
    TAGMETADATA, "TagMetadataDefinition".to_string(),
    TRAIT, "DestinyTraitDefinition".to_string(),
    TRAITCATEGORY, "DestinyTraitCategoryDefinition".to_string(),
    UNLOCK, "DestinyUnlockDefinition".to_string()
});