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
    let id1 = commands
        .spawn_empty()
        .id();
    let id2 = commands
        .spawn_empty()
        .id();
    let id3 = commands
        .spawn_empty()
        .id();

    commands
        .on_undo_builder()
        .add_entity(id1)
        .add_entity(id2)
        .add_entity(id3)
        .on_undo(|_, (entity1, entity2, entity3)| {
            println!("undo entity1 = {entity1:?} entity2 = {entity2:?} entity3 = {entity3:?}");
        });
}


fn keycode_undo(
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if key.just_pressed(KeyCode::R) {
        commands.undo();
    }
}