use crate::map::{GameMap, TileType};
use bevy::ecs::Commands;


fn load_map(map_filename: String) -> GameMap {
    let mut tiles: Vec<TileType> = Vec::new();
    let raw_string = std::fs::read_to_string(map_filename).unwrap();
    let lines = raw_string.lines();
    println!("{}", raw_string);
    lines.for_each(|line|{
        let chars = line.chars();
        chars.for_each(|char| {
            tiles.push(match char {
                'O' => TileType::Ocean,
                'L' => TileType::Land,
                'B' => TileType::Beach,
                _ => TileType::Ocean
            });
        })
    });
    GameMap::from_tiles(tiles)
}

pub fn setup_game_map(commands: &mut Commands) {

    let game_map = load_map(String::from("assets/maps/starting_map.txt"));

    commands.insert_resource(game_map);
}