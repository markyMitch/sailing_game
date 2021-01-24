use bevy::prelude::*;
use crate::config::{GameConfig, GameState};
use crate::player::Player;
use bevy::utils::Duration;

pub struct WoodResource;

struct WindDir;

pub struct RootUiNode;

pub struct TimeCounter;

pub fn setup_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_name = "chancery.ttf";

    commands
        // root node
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        }).with(RootUiNode)
        .with_children(|parent| {
            parent
                // left vertical fill (border)
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        // left vertical fill (content)
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    size: Size::new(Val::Percent(0f32), Val::Percent(10f32)),
                                    ..Default::default()
                                },
                                text: Text {
                                    value: "ShipWreckd".to_string(),
                                    font: asset_server.load(font_name),
                                    style: TextStyle {
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                },
                                ..Default::default()
                            })
                            .spawn(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(0.0)),
                                    size: Size::new(Val::Percent(0f32), Val::Percent(30f32)),
                                    ..Default::default()
                                },
                                text: Text {
                                    value: "Resources:".to_string(),
                                    font: asset_server.load(font_name),
                                    style: TextStyle {
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                },
                                ..Default::default()
                            })
                            .spawn(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(0.0)),
                                    size: Size::new(Val::Percent(0f32), Val::Percent(40f32)),
                                    ..Default::default()
                                },
                                text: Text {
                                    value: "Wood: 0".to_string(),
                                    font: asset_server.load(font_name),
                                    style: TextStyle {
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                },
                                ..Default::default()
                            }).with(WoodResource)
                                .spawn(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(0.0)),
                                        size: Size::new(Val::Percent(0f32), Val::Percent(60f32)),
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Elapsed time\n (secs): ".to_string(),
                                        font: asset_server.load(font_name),
                                        style: TextStyle {
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                }).with(TimeCounter);
                        });
                });



        });
}

pub fn update_resource_counts(mut wood_query: Query<(&WoodResource, &mut Text)>,
                              mut time_query: Query<(&TimeCounter, &mut Text)>,
                              player: Res<Player>, time: Res<Time>, config: Res<GameConfig>) {
    match config.game_state {
        GameState::Running => {
            let new_wood_text = String::from("Wood: ") + &*player.resources.wood.to_string();
            let new_time_text = String::from("Elapsed time (secs):\n") + &*time.time_since_startup().as_secs().to_string();
            let (_, mut text) = wood_query.iter_mut().next().unwrap();
            text.value = new_wood_text;
            let(_, mut time_text) = time_query.iter_mut().next().unwrap();
            time_text.value = new_time_text;
        },
        _ => ()
    }

}

pub fn show_messages(asset_server: Res<AssetServer>, mut config: ResMut<GameConfig>,
                     mut query: Query<(&RootUiNode, &mut NodeBundle)>, commands: &mut Commands,
                     mut materials: ResMut<Assets<ColorMaterial>>) {
    //let (root, mut node) = query.iter_mut().next().unwrap();
    let font_name = "chancery.ttf";
    match config.game_state {
        GameState::PlayerDeath | GameState::GameWon => {
            let mut message = String::from(""); // placeholder
            if config.message_buffer.len() > 0 {
                message = config.message_buffer.iter().next().unwrap().clone();
            }
            commands.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(60.0), Val::Percent(60.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Percent(20.0),

                        top: Val::Percent(20.0),

                        ..Default::default()
                    },
                    border: Rect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                material: materials.add(Color::TEAL.into()),
                ..Default::default()
            })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            position_type: PositionType::Absolute,
                            position: Rect {
                                left: Val::Percent(0.0),

                                top: Val::Percent(0.0),

                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: message,
                            font: asset_server.load(font_name),
                            style: TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                });
        },
        _ => ()
    }
}