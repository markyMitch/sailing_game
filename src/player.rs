use bevy::prelude::Vec2;
use bevy::ecs::Commands;

pub enum Direction {
    North,
    South,
    East,
    West
}

pub struct Player {
    pub x_pos: u32,
    pub y_pos: u32,
    pub direction: Direction
}

impl Player {
    fn new() -> Player {
        // TODO use actual bevy vecs here
        Player{x_pos: 20, y_pos: 20, direction: Direction::North}
    }
}

pub fn setup_game_player(commands: &mut Commands) {
    commands.insert_resource(Player::new());
}