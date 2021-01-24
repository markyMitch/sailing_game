use bevy::prelude::Vec2;
use bevy::ecs::Commands;

pub enum Direction {
    North,
    South,
    East,
    West
}

pub enum TravelState {
    Foot,
    Boat,
    Transition,
}

pub struct PlayerResources {
    pub wood: u32,
}

pub struct Player {
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_delta: i32,
    pub y_delta: i32,
    pub direction: Direction,
    pub travel_state: TravelState,
    pub resources: PlayerResources,
}

impl Player {
    const DEFAULT_TRAVEL_STATE: TravelState = TravelState::Boat;
    fn new() -> Player {
        // TODO use actual bevy vecs here
        let resources : PlayerResources = PlayerResources{wood: 0};
        Player{x_pos: 20, y_pos: 20, x_delta:0, y_delta: 0, direction: Direction::North, travel_state: Player::DEFAULT_TRAVEL_STATE, resources }
    }

    pub fn update_position(&mut self) {
        if self.x_pos + self.x_delta >= 0 {
            // safety check that we wont drop pos below 0
            // tODO improve this
            self.x_pos += self.x_delta;
        }

        if self.y_pos + self.y_delta >= 0 {
            self.y_pos += self.y_delta;
        }
        self.wipe_move_delta();
    }

    pub fn wipe_move_delta(&mut self) {
        self.x_delta = 0;
        self.y_delta = 0;
    }

    pub fn update_wood_resource(&mut self, amount_of_wood: u32) {
        self.resources.wood += amount_of_wood;
    }
}

pub fn setup_game_player(commands: &mut Commands) {
    commands.insert_resource(Player::new());
}

