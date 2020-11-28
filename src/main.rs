use std::io::Read;

use fastnbt::de::from_bytes;
use flate2::read::GzDecoder;

// mod level;
// use level::Level;

// mod map;
// use map::Map;

mod player;
use player::PlayerMapNumbers;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let file = std::fs::File::open(args[0].clone()).unwrap();

    let mut decoder = GzDecoder::new(file);
    let mut data = vec![];

    decoder.read_to_end(&mut data).unwrap();

    // let map: Map = from_bytes(data.as_slice()).unwrap();
    // let level: Level = from_bytes(data.as_slice()).unwrap();
    let player_map_numbers: PlayerMapNumbers = from_bytes(data.as_slice()).unwrap();

    // println!("{:#?}", map);
    // println!("{:#?}", level);
    println!("{:#?}", player_map_numbers);
}
