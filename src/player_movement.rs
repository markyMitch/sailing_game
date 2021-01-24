use bevy::prelude::*;
use crate::player::{Player, TravelState};
use crate::map::{GameMap, TileType};
use crate::config::{GameConfig, GameState};
use crate::view::{WorldTile, TileMaterials};


pub fn resolve_player_movement(mut player: ResMut<Player>, mut world: ResMut<GameMap>, materials: Res<TileMaterials>,
                               mut config: ResMut<GameConfig>, mut world_tiles_query: Query<(&WorldTile, &mut Handle<ColorMaterial>)>) {
    // check to see the type of tile the player is now on
    match config.game_state {
        GameState::Running => {
            let projected_x = (player.x_pos + player.x_delta).abs() as u32;
            let projected_y = (player.y_pos + player.y_delta).abs() as u32;
            let tile_type = world.get_tile(projected_x.clone(), projected_y.clone());

            let (valid_move, message) = match tile_type {
                TileType::Land => {
                    match player.travel_state {
                        TravelState::Boat => {
                            (false, "Game over!\n\nYou crashed upon the land (try landing on a beach next time)")
                        },
                        _ => {
                            player.travel_state = TravelState::Foot;
                            player.update_position();
                            (true, "")
                        }
                    }
                },
                TileType::Beach => {
                    player.travel_state = TravelState::Transition;
                    player.update_position();
                    (true, "")
                },
                TileType::Ocean => {
                    match player.travel_state {
                        TravelState::Foot => {

                            (false, "Game over! \n\nYou fell into the sea (try getting into your boat on a beach next time!)")
                        },
                        _ => {
                            player.travel_state = TravelState::Boat;
                            player.update_position();
                            (true, "")
                        }
                    }
                },
                TileType::Trees => {
                    match player.travel_state {
                        TravelState::Boat => {
                            (true, "")
                        },
                        _ => {
                            player.travel_state = TravelState::Foot;
                            player.update_position();
                            player.update_wood_resource(1);
                            world.set_tile(projected_x, projected_y, TileType::Land);
                            for (world_tile, mut mat_handle) in world_tiles_query.iter_mut() {
                                if(world_tile.x_val == projected_x) && (world_tile.y_val == projected_y) {
                                    *mat_handle = materials.land_material.clone();
                                    break;
                                }
                            }
                            (true, "")
                        }
                    }
                }
                _ => (true, "")
            };

            // now wipe player's delta
            player.wipe_move_delta();

            if !valid_move {
                config.game_state = GameState::PlayerDeath;
                config.message_buffer.push(String::from(message));
            }
        },
        _ => ()
    };



}

pub fn check_win_condition(player: Res<Player>, world: Res<GameMap>, mut config: ResMut<GameConfig>) {
    if player.resources.wood >= world.total_resource_tiles {
        let win_message = "
Congrats, you have collected all the resource tiles.

Why not try again and see if you can beat your best time?
        ";
        config.game_state = GameState::GameWon;
        config.message_buffer.push(String::from(win_message));
    }
}