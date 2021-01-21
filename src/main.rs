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

mod map;
mod player;
mod view;

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin);


    app.add_startup_system(setup_cameras.system())
        .add_startup_system(map::setup_game_map.system())
        .add_startup_system(player::setup_game_player())
        .add_startup_system(view::setup_materials.system())
        .add_startup_stage("game_setup", SystemStage::single(view::draw_view.system()))
        .run();
}

fn setup_cameras(commands: &mut Commands,) {
    commands // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}