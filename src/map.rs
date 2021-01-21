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
    pub const WIDTH: u32 = 200;
    pub const HEIGHT: u32 = 200;

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

    pub fn set_tile(&mut self, x: u32, y: u32, tile_type: TileType) {
        let index = (GameMap::WIDTH * y) + x;
        self.tiles[index as usize] = tile_type;
    }
}

struct Position {
    x: u32,
    y: u32
}

pub fn setup_game_map(commands: &mut Commands) {
    let mut game_map = GameMap::new();

    // make some islands
    // TODO: make this better
    let centre = Position{x: 25, y:25};
    game_map.set_tile(centre.x, centre.y, TileType::Land);


    commands.insert_resource(game_map);
}

