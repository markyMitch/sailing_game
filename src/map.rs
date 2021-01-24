use bevy::reflect::List;
use bevy::prelude::*;

pub enum TileType {
    Ocean,
    Land,
    Beach,
    Trees,
}



pub struct GameMap {
    tiles: Vec<TileType>,
    pub total_resource_tiles: u32
}

impl GameMap {
    pub const WIDTH: u32 = 40;
    pub const HEIGHT: u32 = 40;

    pub fn new() -> GameMap {
        let mut vector: Vec<TileType> = Vec::new();
        for _ in 0..(GameMap::WIDTH * GameMap::HEIGHT) {
            vector.push(TileType::Ocean);
        }
        GameMap {tiles: vector, total_resource_tiles: 0u32}
    }

    pub fn from_tiles(tiles: Vec<TileType>) -> GameMap {
        let num_resources = tiles.iter().filter(|&tile| {
            match tile {
                TileType::Trees => true,
                _ => false
            }
        }).count();

        GameMap{tiles: tiles, total_resource_tiles: num_resources as u32 }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> &TileType {
        let index = (GameMap::WIDTH * y) + x;
        return self.tiles.get(index as usize).unwrap();
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile_type: TileType) {
        let index = (GameMap::WIDTH * y) + x;
        self.tiles[index as usize] = tile_type;
    }

    pub fn get_tiles(&self) -> &Vec<TileType> {
        &self.tiles
    }
}

struct Position {
    x: u32,
    y: u32
}



