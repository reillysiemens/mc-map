use fastnbt::Value;
use serde::de::{self, Deserializer, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use std::fmt;

struct ItemStructure(Option<i32>);

struct ItemStructureVisitor;

impl<'de> Visitor<'de> for ItemStructureVisitor {
    type Value = ItemStructure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("item structure NBT data")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut found_id: bool = false;
        let mut tag: Option<Value> = None;

        while let Some(key) = map.next_key::<&str>()? {
            if key == "id" {
                let value: &str = map.next_value()?;

                if value != "minecraft:filled_map" {
                    return Ok(ItemStructure(None));
                }

                found_id = true;
            }

            if key == "tag" {
                tag = Some(map.next_value()?);
            }

            if found_id && tag.is_some() {
                break;
            }
        }

        if !found_id {
            return Err(de::Error::missing_field("id"));
        }

        match tag {
            Some(Value::Compound(compound)) => Ok(ItemStructure(None)),
            Some(value) => Err(de::Error::invalid_type(
                Unexpected::Other(&format!("{:?}", value)),
                &"",
            )),
            None => Err(de::Error::missing_field("tag")),
        }
    }
}

impl<'de> Deserialize<'de> for ItemStructure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ItemStructureVisitor)
    }
}

// TODO: Change this to PlayerMapNumbers.
#[derive(Debug)]
pub struct PlayerMapIds(Vec<i32>);

struct PlayerMapIdsVisitor;

impl<'de> Visitor<'de> for PlayerMapIdsVisitor {
    type Value = PlayerMapIds;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("player NBT data")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map_ids = Vec::new();

        while let Some(key) = map.next_key::<&str>()? {
            if key == "Inventory" || key == "EnderItems" {
                let item_structure: Vec<ItemStructure> = map.next_value()?;
            }
        }

        Ok(PlayerMapIds(map_ids))
    }
}

impl<'de> Deserialize<'de> for PlayerMapIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PlayerMapIdsVisitor)
    }
}
