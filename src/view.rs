use bevy::window::Windows;
use bevy::ecs::Res;
use bevy::prelude::*;

use crate::map::{TileType, GameMap};
use crate::player::Player;

pub struct MainCamera;
pub struct PlayerSprite;

pub fn setup_cameras(commands: &mut Commands,) {
    commands // cameras
        .spawn(Camera2dBundle::default()).with(MainCamera)
        .spawn(CameraUiBundle::default());
}

pub struct TileMaterials {
    ocean_material : Handle<ColorMaterial>,
    land_material : Handle<ColorMaterial>,
    beach_material : Handle<ColorMaterial>,
}

pub struct WorldMaterials {
    player_material : Handle<ColorMaterial>
}

pub fn setup_materials(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>,
                       asset_server: Res<AssetServer>) {
    commands.insert_resource(TileMaterials {
        ocean_material: materials.add(Color::rgb(0f32, 0f32, 1.0).into()),
        land_material: materials.add(Color::rgb(0f32, 1.0, 0f32).into()),
        beach_material: materials.add(Color::rgb(0f32, 0.5, 0.5).into()),
    });

    let player_icon = asset_server.load("player_icon.png");
    commands.insert_resource(WorldMaterials {
        player_material: materials.add(player_icon.into())
    });
}

pub fn get_tile_material(tile: &TileType, materials: &Res<TileMaterials>) -> Handle<ColorMaterial>
{
    match tile {
        TileType::Ocean => materials.ocean_material.clone(),
        TileType::Land => materials.land_material.clone(),
        TileType::Beach => materials.beach_material.clone(),
    }
}

pub fn draw_view(commands: &mut Commands, windows: Res<Windows>, tile_materials: Res<TileMaterials>,
                 map: Res<GameMap>, player: Res<Player>, world_materials: Res<WorldMaterials>)
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

    // can we just spawn the entire map?
    for x_coord in 0..GameMap::WIDTH {
        for y_coord in 0..GameMap::HEIGHT {
            let mut tile_sb = SpriteBundle {
                material: get_tile_material(map.get_tile(x_coord, y_coord), &tile_materials),
                sprite: Sprite::new(Vec2::new(tile_height, tile_height)),
                ..Default::default()
            };
            tile_sb.transform.translation = Vec3 {x: tile_height * (x_coord as f32), y: tile_height * (y_coord as f32), z:0f32 };
            commands.spawn(tile_sb);
        }
    }

    // spawn player
    let mut player_sb = SpriteBundle {
        material: world_materials.player_material.clone(),
        sprite: Sprite::new(Vec2::new(tile_height, tile_height)),
        ..Default::default()
    };
    player_sb.transform.translation = Vec3 {x: tile_height * (player.x_pos as f32), y: tile_height * (player.y_pos as f32), z:5f32 };
    commands.spawn(player_sb).with(PlayerSprite);
}

pub fn update_view(windows: Res<Windows>, mut camera_query: Query<(&mut MainCamera, &mut Transform)>,
                   mut player_query: Query<(&mut PlayerSprite, &mut Transform)>,
                   player: ResMut<Player>) {
    const TILES_ON_SCREEN : u32 = 40;
    let window = windows.get_primary().unwrap();
    let shorter_side = window.width().min(window.height());
    let tile_height = shorter_side / TILES_ON_SCREEN as f32;
    let (mut player_sprite, mut player_sprite_transform) = player_query.iter_mut().next().unwrap();
    player_sprite_transform.translation.x = (player.x_pos as f32) * tile_height;
    player_sprite_transform.translation.y = (player.y_pos as f32) * tile_height;
    let (mut m
        , mut main_camera_transform) = camera_query.iter_mut().next().unwrap();
    let current_depth = main_camera_transform.translation.z;
    main_camera_transform.translation = Vec3 {
        x: (player.x_pos as f32) * tile_height,
        y: (player.y_pos as f32) * tile_height,
        z: current_depth};
}