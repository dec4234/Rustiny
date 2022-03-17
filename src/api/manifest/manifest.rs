use crate::api::ApiClient::ApiClient;
use anyhow::Result;
use serde_json::Value;
use crate::api::DestinyAPI::URL_BASE;
use serde::{Deserialize, Serialize};

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
        let resp = self.client.get(format!("{}/Destiny2/Manifest/{entityType}/{hashIdentifier}/", URL_BASE, entityType = typ.get_type(), hashIdentifier = hash)).await?;

        Ok(resp)
    }

    pub async fn manifest_get(&self, typ: ManifestEntityType, hash: String) -> Result<String> {
        let resp = self.client.get(format!("{}/Destiny2/Manifest/{entityType}/{hashIdentifier}/", URL_BASE, entityType = typ.get_type(), hashIdentifier = hash)).await?;

        Ok(resp)
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

pub enum ManifestEntityType {
    // Relating to items/character's inventory
    ARTIFACT,
    BREAKERTYPE,
    COLLECTIBLE,
    EQUIPMENTSLOT,
    INVENTORYBUCKET,
    /** Any item that can go in a user's invenory such as armor, weapons, and planetary materials */
    INVENTORYITEM,
    ITEMCATEGORY,
    ITEMSTAT,
    ITEMSTATGROUP,
    ITEMTIER,
    MATERIALREQUIREMENTSET,
    POWERCAP,
    /** Record is another word for Triumph **/
    RECORD,
    REWARDSOURCE,
    SANDBOXPERK,
    TALENTGRID,
    REWARDENTRY,

    // Relating to users
    CLASS,
    GENDER,
    MILESTONE,
    PROGRESSION,
    RACE,

    // Vendors/world
    ACTIVITY,
    ACTIVITYGRAPH,
    ACTIVITYMODE,
    ACTIVITYMODIFIER,
    ACTIVITYTYPE,

    DAMAGETYPE,
    DESTINATION,
    FACTION,
    LOCATION,
    OBJECTIVE,
    PLACE,
    VENDOR,
    VENDORGROUP,

    // Misc
    CHECKLIST,
    ENERGYTYPE,
    HISTORICALSTATS,
    PRESENTATIONNODE,
    LORE,
    METRIC,
    PLUGSET,
    REPORTREASONCATEGORY,
    SEASON,
    SEASONPASS,
    SOCKETCATEGORY,
    SOCKETTYPE,
    TAGMETADATA,
    TRAIT,
    TRAITCATEGORY,
    UNLOCK,

}

impl ManifestEntityType {
    pub fn get_type(&self) -> &str {
        match self {
            ManifestEntityType::ARTIFACT => { "DestinyArtifactDefinition" }
            ManifestEntityType::BREAKERTYPE => { "DestinyBreakerTypeDefinition" }
            ManifestEntityType::COLLECTIBLE => { "DestinyCollectibleDefinition" }
            ManifestEntityType::EQUIPMENTSLOT => { "DestinyEquipmentSlotDefinition" }
            ManifestEntityType::INVENTORYBUCKET => { "DestinyInventoryBucketDefinition" }

            ManifestEntityType::INVENTORYITEM => { "DestinyInventoryItemDefinition" }
            ManifestEntityType::ITEMCATEGORY => { "DestinyItemCategoryDefinition" }
            ManifestEntityType::ITEMSTAT => { "DestinyStatDefinition" }
            ManifestEntityType::ITEMSTATGROUP => { "DestinyStatGroupDefinition" }
            ManifestEntityType::ITEMTIER => { "DestinyItemTierTypeDefinition" }
            ManifestEntityType::MATERIALREQUIREMENTSET => { "DestinyMaterialRequirementSetDefinition" }
            ManifestEntityType::POWERCAP => { "DestinyPowerCapDefinition" }
            ManifestEntityType::RECORD => { "DestinyRecordDefinition" }
            ManifestEntityType::REWARDSOURCE => { "DestinyRewardSourceDefinition" }
            ManifestEntityType::SANDBOXPERK => { "DestinySandboxPerkDefinition" }
            ManifestEntityType::TALENTGRID => { "DestinyTalentGridDefinition" }
            ManifestEntityType::REWARDENTRY => { "DestinyMilestoneRewardEntryDefinition" }

            ManifestEntityType::CLASS => { "DestinyClassDefinition" }
            ManifestEntityType::GENDER => { "DestinyGenderDefinition" }
            ManifestEntityType::MILESTONE => { "DestinyMilestoneDefinition" }
            ManifestEntityType::PROGRESSION => { "DestinyProgressionDefinition" }
            ManifestEntityType::RACE => { "DestinyRaceDefinition" }

            ManifestEntityType::ACTIVITY => { "DestinyActivityDefinition" }
            ManifestEntityType::ACTIVITYGRAPH => { "DestinyActivityGraphDefinition" }
            ManifestEntityType::ACTIVITYMODE => { "DestinyActivityModeDefinition" }
            ManifestEntityType::ACTIVITYMODIFIER => { "DestinyActivityModifierDefinition" }
            ManifestEntityType::ACTIVITYTYPE => { "DestinyActivityTypeDefinition" }

            ManifestEntityType::DAMAGETYPE => { "DestinyDamageTypeDefinition" }
            ManifestEntityType::DESTINATION => { "DestinyDestinationDefinition" }
            ManifestEntityType::FACTION => { "DestinyFactionDefinition" }
            ManifestEntityType::LOCATION => { "DestinyLocationDefinition" }
            ManifestEntityType::OBJECTIVE => { "DestinyObjectiveDefinition" }
            ManifestEntityType::PLACE => { "DestinyPlaceDefinition" }
            ManifestEntityType::VENDOR => { "DestinyVendorDefinition" }
            ManifestEntityType::VENDORGROUP => { "DestinyVendorGroupDefinition" }

            ManifestEntityType::CHECKLIST => { "DestinyChecklistDefinition" }
            ManifestEntityType::ENERGYTYPE => { "DestinyEnergyTypeDefinition" }
            ManifestEntityType::HISTORICALSTATS => { "DestinyHistoricalStatsDefinition" }
            ManifestEntityType::PRESENTATIONNODE => { "DestinyPresentationNodeDefinition" }
            ManifestEntityType::LORE => { "DestinyLoreDefinition" }
            ManifestEntityType::METRIC => { "DestinyMetricDefinition" }
            ManifestEntityType::PLUGSET => { "DestinyPlugSetDefinition" }
            ManifestEntityType::REPORTREASONCATEGORY => { "DestinyReportReasonCategoryDefinition" }
            ManifestEntityType::SEASON => { "DestinySeasonDefinition" }
            ManifestEntityType::SEASONPASS => { "DestinySeasonPassDefinition" }
            ManifestEntityType::SOCKETCATEGORY => { "DestinySocketCategoryDefinition" }
            ManifestEntityType::SOCKETTYPE => { "DestinySocketTypeDefinition" }
            ManifestEntityType::TAGMETADATA => { "TagMetadataDefinition" }
            ManifestEntityType::TRAIT => { "DestinyTraitDefinition" }
            ManifestEntityType::TRAITCATEGORY => { "DestinyTraitCategoryDefinition" }
            ManifestEntityType::UNLOCK => { "DestinyUnlockDefinition" }
        }
    }
}