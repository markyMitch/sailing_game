use bevy::prelude::*;

pub enum GameState {
    Running,
    PlayerDeath,
    Setup,
    GameWon,
}

pub struct GameConfig {
    pub message_buffer: Vec<String>,
    pub game_state: GameState
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            message_buffer: Vec::new(),
            game_state: GameState::Running
        }
    }
}

pub fn create_config(commands: &mut Commands) {
    commands.insert_resource(GameConfig::default());
}