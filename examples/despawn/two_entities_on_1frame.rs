use bevy::prelude::*;

use bevy_undo::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UndoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        "Press [R]: do undo",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 80.0,
            color: Color::WHITE,
        },
    ));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE_RED,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        })
        .on_undo_with_entity_commands(|cmd| {
            cmd.despawn();
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .on_undo_with_entity_commands(|cmd| {
            cmd.despawn();
        });
}

fn keyboard_input_system(mut commands: Commands, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::R) {
        commands.undo();
        commands.undo();
    }
}
