use fastnbt::Value;
use serde::de::{self, Deserializer, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
struct ItemStructure(Option<i64>);

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
            match key {
                "id" => {
                    let value: &str = map.next_value()?;

                    if value == "minecraft:filled_map" {
                        found_id = true;
                    }
                }
                "tag" => {
                    tag = Some(map.next_value()?);
                }
                _ => {
                    map.next_value::<Value>()?;
                }
            }
        }

        if !found_id {
            return Ok(ItemStructure(None));
        }

        match tag {
            Some(Value::Compound(compound)) => match compound["map"] {
                Value::Integral(map) => Ok(ItemStructure(Some(map))),
                ref value => Err(de::Error::invalid_type(
                    Unexpected::Other(&format!("{:?}", value)),
                    &"an integer",
                )),
            },
            Some(value) => Err(de::Error::invalid_type(
                Unexpected::Other(&format!("{:?}", value)),
                &"a compound",
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
pub struct PlayerMapIds(Vec<i64>);

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
                map_ids.extend(item_structure.into_iter().filter_map(|item| item.0));
            } else {
                map.next_value::<Value>()?;
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
