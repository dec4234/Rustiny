use crate::api::ApiClient::ApiClient;
use anyhow::Result;
use crate::api::DestinyAPI::URL_BASE;

pub struct Manifest {
    client: ApiClient,
}

impl Manifest {
    pub fn new(client: ApiClient) -> Self {
        Self {
            client,
        }
    }

    pub async fn manifest_get(&self, typ: ManifestEntityType, hash: String) -> Result<String> {
        let resp = self.client.get(format!("{}/Destiny2/Manifest/{entityType}/{hashIdentifier}/", URL_BASE, entityType = typ.get_type(), hashIdentifier = hash)).await?;

        Ok(resp)
    }
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