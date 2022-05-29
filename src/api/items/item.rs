use anyhow::Result;
use serde_json::{json, Value};
use crate::api::ApiClient::ApiClient;
use crate::api::DestinyAPI::URL_BASE;
use crate::DestinyCharacter;

#[derive(Debug, Clone)]
pub struct Item {
    pub hash: i64,
}

impl Item {
    /**
    Use manifest to get the item under DestinyInventoryItemDefinition
    **/
    pub fn new(hash: i64) -> Self {
        todo!()
    }

    pub async fn search(client: ApiClient, search: String) -> Result<Vec<Self>> {
        let resp = client.get(format!("{base}/Destiny2/Armory/Search/DestinyInventoryItemDefinition/{search}/", base = URL_BASE)).await?;

        todo!()
    }
}

pub struct InventoryItem {
    pub item: Item,
    pub character_owner: DestinyCharacter,
    pub instanceID: i64,
    pub isEquippable: bool,
    pub stackSize: i32,
}

impl InventoryItem {
    pub fn get_json(&self, move_to_vault: bool) -> Value {
        json!({
            "itemReferenceHash": self.item.hash,
            "stackSize": self.stackSize,
            "transferToVault": move_to_vault,
            "itemId": self.instanceID,
            "characterId": self.character_owner.characterId.parse::<i64>().unwrap(),
            "membershipType": self.character_owner.membershipType,
        })
    }
}

pub trait Equippable {
    fn equip(&self) -> Result<()>;

    fn transfer_to_vault(&self) -> Result<()>;

    fn transfer_to_character(&self) -> Result<()>;
}