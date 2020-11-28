use serde::de::Deserializer;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Version<'a> {
    #[serde(borrow)]
    name: &'a str,
}

fn tuple_from_spawn<'de, D>(deserializer: D) -> Result<(i32, i32, i32), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct Spawn {
        #[serde(rename = "SpawnX")]
        x: i32,
        #[serde(rename = "SpawnY")]
        y: i32,
        #[serde(rename = "SpawnZ")]
        z: i32,
    }

    let spawn = Spawn::deserialize(deserializer)?;
    Ok((spawn.x, spawn.y, spawn.z))
}

#[derive(Deserialize, Debug)]
pub struct LevelData<'a> {
    #[serde(borrow, rename = "Version")]
    version: Version<'a>,
    #[serde(flatten, deserialize_with = "tuple_from_spawn")]
    spawn: (i32, i32, i32),
}

#[derive(Deserialize, Debug)]
pub struct Level<'a> {
    #[serde(borrow, rename = "Data")]
    data: LevelData<'a>,
}
