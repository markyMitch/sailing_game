use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin, EventQueue};
use bevy_rapier2d::render::RapierRenderPlugin;

use core::f32::consts;

use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder,
    geometry::ColliderBuilder,
    pipeline::PhysicsPipeline,
};
use rapier2d::dynamics::RigidBody;
use bevy_rapier2d::na::{Isometry, Isometry2};
use crate::ui::setup_ui;
use crate::config::create_config;

mod map;
mod player;
mod view;
mod input;
mod map_loader;
mod ui;
mod player_movement;
mod config;
mod maps;

fn main() {

    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin);


    app.add_startup_system(view::setup_cameras.system())
        .add_startup_system(map_loader::setup_game_map.system())
        .add_startup_system(player::setup_game_player.system())
        .add_startup_system(view::setup_materials.system())
        .add_startup_system(setup_ui.system())
        .add_startup_system(create_config.system())
        .add_startup_stage("game_setup", SystemStage::single(view::draw_view.system()))

        .add_system(input::input_system.system())
        .add_system(player_movement::resolve_player_movement.system())
        .add_system(view::update_view.system())

        .add_system(ui::update_resource_counts.system())
        .add_system(ui::show_messages.system())
        .add_system(player_movement::check_win_condition.system())

        .run();
}



