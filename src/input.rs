use bevy::prelude::*;
use crate::player::Player;

pub fn input_system(keyboard_input: Res<Input<KeyCode>>, mut player: ResMut<Player>) {

    if keyboard_input.pressed(KeyCode::W) {
        player.y_delta += 1;
    }
    if keyboard_input.pressed(KeyCode::S) {
        player.y_delta -= 1;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player.x_delta += 1;
    }
    if keyboard_input.pressed(KeyCode::A) {
        player.x_delta -= 1;
    }
}
