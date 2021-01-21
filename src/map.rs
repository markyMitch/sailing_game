use bevy::reflect::List;
use bevy::prelude::*;

pub enum TileType {
    Ocean,
    Land,
    Beach,
}



pub struct GameMap {

    tiles: Vec<TileType>
}

impl GameMap {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 200;

    pub fn new() -> GameMap {
        let mut vector: Vec<TileType> = Vec::new();
        for _ in 0..(GameMap::WIDTH * GameMap::HEIGHT) {
            vector.push(TileType::Ocean);
        }
        GameMap {tiles: vector}
    }

    pub fn get_tile(&self, x: u32, y: u32) -> &TileType {
        let index = (GameMap::WIDTH * y) + x;
        return self.tiles.get(index as usize).unwrap();
    }
}

pub fn setup_game_map(commands: &mut Commands) {
    commands.insert_resource(GameMap::new());
}

