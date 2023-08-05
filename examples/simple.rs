use bevy::prelude::*;
use bevy_undo::prelude::*;

/// Here is a simple example that prints `Undo!!!!!!` when you press the R key.when input key `R`
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UndoPlugin) // Please add `UndoPlugin`
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
    if key.just_pressed(KeyCode::R) {
        commands.undo();
    }
}
