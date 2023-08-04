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
        .on_undo_with_entity_commands(|cmd| {
            cmd.despawn();
        });
}


fn keyboard_input_system(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
) {
    if key.pressed(KeyCode::R) {
        commands.undo();
    }
}
