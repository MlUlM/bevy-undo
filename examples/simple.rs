use bevy::prelude::*;

use bevy_undo::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UndoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keycode_undo)
        .run();
}


fn setup(
    mut commands: Commands
) {
    commands
        .on_undo(|_| {
            println!("Undo!!!!!!");
        });
}


fn keycode_undo(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
) {
    if key.pressed(KeyCode::R) {
        commands.undo();
    }
}
