use bevy::prelude::*;

use bevy_undo::extension::{CommandsOnUndoExt, CommandsUndoExt};
use bevy_undo::UndoPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UndoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}


fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE_RED,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        })
        .on_undo(|cmd| {
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
        .on_undo(|cmd| {
            cmd.despawn();
        });
}


fn keyboard_input_system(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::R) {
        commands.undo();
    }
}
