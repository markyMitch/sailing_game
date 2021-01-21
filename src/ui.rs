use bevy::prelude::*;

struct UiText;

pub fn setup_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
        })
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
                                    size: Size::new(Val::Percent(5f32), Val::Percent(10f32)),
                                    ..Default::default()
                                },
                                text: Text {
                                    value: "ShipWreckd".to_string(),
                                    font: asset_server.load("fonts/chancery.ttf"),
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
                                    margin: Rect::all(Val::Px(5.0)),
                                    size: Size::new(Val::Percent(5f32), Val::Percent(30f32)),
                                    ..Default::default()
                                },
                                text: Text {
                                    value: "Resources:".to_string(),
                                    font: asset_server.load("fonts/chancery.ttf"),
                                    style: TextStyle {
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                },
                                ..Default::default()
                            });
                        });
                });

                // // absolute positioning
                // .spawn(NodeBundle {
                //     style: Style {
                //         size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                //         position_type: PositionType::Absolute,
                //         position: Rect {
                //             left: Val::Px(210.0),
                //             bottom: Val::Px(10.0),
                //             ..Default::default()
                //         },
                //         border: Rect::all(Val::Px(20.0)),
                //         ..Default::default()
                //     },
                //     material: materials.add(Color::rgb(0.4, 0.4, 1.0).into()),
                //     ..Default::default()
                // })
                // .with_children(|parent| {
                //     parent.spawn(NodeBundle {
                //         style: Style {
                //             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                //             ..Default::default()
                //         },
                //         material: materials.add(Color::rgb(0.8, 0.8, 1.0).into()),
                //         ..Default::default()
                //     });
                // });


        });
}