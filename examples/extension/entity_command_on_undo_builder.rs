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

fn setup(mut commands: Commands) {
    let id1 = commands.spawn_empty().id();

    commands
        .spawn_empty()
        .on_undo_builder()
        .add_entity(id1)
        .on_undo(|_, (entity1, entity2)| {
            println!("undo entity1 = {entity1:?} entity2 = {entity2:?}");
        });
}

fn keycode_undo(key: Res<Input<KeyCode>>, mut commands: Commands) {
    if key.just_pressed(KeyCode::R) {
        commands.undo();
    }
}
