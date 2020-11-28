use fastnbt::de::from_bytes;
use flate2::read::GzDecoder;
use serde::de::{Deserializer, Error};
use serde::Deserialize;
use serde_json;
use std::convert::TryFrom;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unknown dimension: {0}")]
struct UnknownDimension(String);

#[derive(Error, Debug)]
#[error("unknown banner color: {0}")]
struct UnknownBannerColor(String);

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

#[derive(Deserialize, Debug)]
struct Map {
    #[serde(rename = "DataVersion")]
    data_version: i32,

    data: MapData,
}

#[derive(Deserialize, Debug)]
struct MapData {
    // TODO: How to make this an enum?
    dimension: Dimension,
    #[serde(rename = "xCenter")]
    x_center: i32,
    #[serde(rename = "zCenter")]
    z_center: i32,

    banners: Vec<Banner>,
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

#[derive(Deserialize, Debug)]
struct Banner {
    #[serde(rename = "Color")]
    color: BannerColor,
    #[serde(rename = "Name", deserialize_with = "text_from_json")]
    name: String,
    #[serde(rename = "Pos")]
    position: Position,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let file = std::fs::File::open(args[0].clone()).unwrap();

    let mut decoder = GzDecoder::new(file);
    let mut data = vec![];

    decoder.read_to_end(&mut data).unwrap();

    let map: fastnbt::error::Result<Map> = from_bytes(data.as_slice());

    println!("{:#?}", map);
}
