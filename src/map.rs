use serde::de::{Deserializer, Error};
use serde::Deserialize;
use serde_json;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unknown dimension: {0}")]
struct UnknownDimension(String);

#[derive(Error, Debug)]
#[error("unknown banner color: {0}")]
struct UnknownBannerColor(String);

#[derive(Error, Debug)]
#[error("unknown scale: {0}")]
struct UnknownScale(i32);

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(try_from = "i32")]
enum Scale {
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl TryFrom<i32> for Scale {
    type Error = UnknownScale;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Scale::Zero),
            1 => Ok(Scale::One),
            2 => Ok(Scale::Two),
            3 => Ok(Scale::Three),
            4 => Ok(Scale::Four),
            unknown => Err(UnknownScale(unknown)),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(try_from = "&str")]
enum Dimension {
    Overworld,
    TheNether,
    TheEnd,
}

impl<'a> TryFrom<&'a str> for Dimension {
    type Error = UnknownDimension;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "minecraft:overworld" => Ok(Dimension::Overworld),
            "minecraft:the_nether" => Ok(Dimension::TheNether),
            "minecraft:the_end" => Ok(Dimension::TheEnd),
            unknown => Err(UnknownDimension(unknown.to_string())),
        }
    }
}

fn tuple_from_map_center<'de, D>(deserializer: D) -> Result<(i32, i32), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct MapCenter {
        #[serde(rename = "xCenter")]
        x: i32,
        #[serde(rename = "zCenter")]
        z: i32,
    }

    let center = MapCenter::deserialize(deserializer)?;
    Ok((center.x, center.z))
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(try_from = "&str")]
enum BannerColor {
    Black,
    Blue,
    Brown,
    Cyan,
    Gray,
    Green,
    LightBlue,
    LightGray,
    Lime,
    Magenta,
    Orange,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
}

impl<'a> TryFrom<&'a str> for BannerColor {
    type Error = UnknownBannerColor;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "black" => Ok(BannerColor::Black),
            "blue" => Ok(BannerColor::Blue),
            "brown" => Ok(BannerColor::Brown),
            "cyan" => Ok(BannerColor::Cyan),
            "gray" => Ok(BannerColor::Gray),
            "green" => Ok(BannerColor::Green),
            "lightblue" => Ok(BannerColor::LightBlue),
            "lightgray" => Ok(BannerColor::LightGray),
            "lime" => Ok(BannerColor::Lime),
            "magenta" => Ok(BannerColor::Magenta),
            "orange" => Ok(BannerColor::Orange),
            "pink" => Ok(BannerColor::Pink),
            "purple" => Ok(BannerColor::Purple),
            "red" => Ok(BannerColor::Red),
            "white" => Ok(BannerColor::White),
            "yellow" => Ok(BannerColor::Yellow),
            unknown => Err(UnknownBannerColor(unknown.to_string())),
        }
    }
}

fn text_from_json<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct BannerName {
        text: String,
    }

    let s: &'de str = Deserialize::deserialize(deserializer)?;
    let banner_name: BannerName = serde_json::from_str(s).map_err(D::Error::custom)?;
    Ok(banner_name.text)
}

fn tuple_from_banner_position<'de, D>(deserializer: D) -> Result<(i32, i32, i32), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    struct BannerPosition {
        x: i32,
        y: i32,
        z: i32,
    }

    let position = BannerPosition::deserialize(deserializer)?;
    Ok((position.x, position.y, position.z))
}

#[derive(Deserialize, Debug)]
struct Banner {
    #[serde(rename = "Color")]
    color: BannerColor,
    #[serde(rename = "Name", deserialize_with = "text_from_json")]
    name: String,
    #[serde(rename = "Pos", deserialize_with = "tuple_from_banner_position")]
    position: (i32, i32, i32),
}

#[derive(Deserialize, Debug)]
struct MapData<'a> {
    scale: Scale,
    dimension: Dimension,
    #[serde(rename = "unlimitedTracking")]
    unlimited_tracking: bool,
    #[serde(flatten, deserialize_with = "tuple_from_map_center")]
    center: (i32, i32),
    banners: Vec<Banner>,
    #[serde(borrow)]
    colors: &'a [u8],
}

#[derive(Deserialize, Debug)]
pub struct Map<'a> {
    #[serde(borrow)]
    data: MapData<'a>,
}
