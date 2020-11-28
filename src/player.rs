use fastnbt::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct InventorySlot<'a> {
    id: &'a str,
    tag: Option<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Player<'a> {
    #[serde(borrow)]
    inventory: Vec<InventorySlot<'a>>,
    ender_items: Vec<InventorySlot<'a>>,
}

// TODO: Implement Deref for this for cleaner access? IntoIterator?
#[derive(Deserialize, Debug)]
#[serde(from = "Player")]
pub struct PlayerMapNumbers(Vec<i64>);

fn maybe_map_id(slot: InventorySlot) -> Option<i64> {
    if slot.id != "minecraft:filled_map" {
        return None;
    }

    match slot.tag {
        Some(Value::Compound(compound)) => match compound["map"] {
            Value::Integral(map) => Some(map),
            _ => None,
        },
        _ => None,
    }
}

impl<'a> From<Player<'a>> for PlayerMapNumbers {
    fn from(player: Player) -> Self {
        PlayerMapNumbers(
            player
                .inventory
                .into_iter()
                .chain(player.ender_items.into_iter())
                .filter_map(maybe_map_id)
                .collect(),
        )
    }
}
