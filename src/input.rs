use bevy::prelude::*;
use crate::player::Player;

pub fn input_system(keyboard_input: Res<Input<KeyCode>>, mut player: ResMut<Player>) {

    if keyboard_input.pressed(KeyCode::W) {
        player.y_pos += 1;
    }
    if keyboard_input.pressed(KeyCode::S) {
        player.y_pos -= 1;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player.x_pos += 1;
    }
    if keyboard_input.pressed(KeyCode::A) {
        player.x_pos -= 1;
    }
}
