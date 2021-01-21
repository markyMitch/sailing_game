use bevy::window::Windows;
use bevy::ecs::Res;
use bevy::prelude::*;

use crate::map::{TileType, GameMap};
use crate::player::Player;

pub struct TileMaterials {
    ocean_material : Handle<ColorMaterial>,
    land_material : Handle<ColorMaterial>,
    beach_material : Handle<ColorMaterial>,
}

pub fn setup_materials(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(TileMaterials {
        ocean_material: materials.add(Color::rgb(0f32, 0f32, 1.0).into()),
        land_material: materials.add(Color::rgb(0f32, 1.0, 0f32).into()),
        beach_material: materials.add(Color::rgb(0f32, 0.5, 0.5).into()),
    });
}

pub fn get_tile_material(tile: &TileType, materials: Res<TileMaterials>) -> Handle<ColorMaterial>
{
    match tile {
        TileType::Ocean => materials.ocean_material.clone(),
        TileType::Land => materials.land_material.clone(),
        TileType::Beach => materials.beach_material.clone(),
    }
}

pub fn draw_view(commands: &mut Commands, windows: Res<Windows>, materials: Res<TileMaterials>,
map: Res<GameMap>, player: Res<Player>)
{
    const TILES_ON_SCREEN : u32 = 40;
    let window = windows.get_primary().unwrap();

    /*
    We need to work out how many squares we can fit on to the window. An easy way to do this for now
    will be to find which dimension is shorter and scale off that
     */
    let shorter_side = window.width().min(window.height());
    let tile_height = shorter_side / TILES_ON_SCREEN as f32;
    let tiles_high: usize = (window.height() / tile_height) as usize;
    let tiles_wide: usize = (window.width() / tile_height) as usize;

    



    commands.spawn(SpriteBundle {
        material: materials.ocean_material.clone(),
        sprite: Sprite::new(Vec2::new(100.0, 100.0)),
        ..Default::default()
    });
}