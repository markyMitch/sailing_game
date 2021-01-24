use crate::map::{GameMap, TileType};
use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::utils::BoxedFuture;
use crate::maps;


fn load_map(raw_string: &str) -> GameMap {
    let mut tiles: Vec<TileType> = Vec::new();

    let lines = raw_string.lines();

    println!("{}", raw_string);
    lines.for_each(|line|{
        let chars = line.chars();
        chars.for_each(|char| {
            tiles.push(match char {
                'O' => TileType::Ocean,
                'L' => TileType::Land,
                'B' => TileType::Beach,
                'T' => TileType::Trees,
                _ => TileType::Ocean
            });
        })
    });
    GameMap::from_tiles(tiles)
}

pub fn setup_game_map(commands: &mut Commands, asset_server: Res<AssetServer>) {

    let game_map = load_map(maps::get_starting_map());

    commands.insert_resource(game_map);
}

// pub struct TextfileLoader {
//
// }
//
// impl AssetLoader for TextfileLoader {
//     fn load<'a>(
//         &'a self,
//         bytes: &'a [u8],
//         load_context: &'a mut LoadContext,
//     ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
//
//     }
//     fn extensions(&self) -> &[&str] {
//
//     }
// }