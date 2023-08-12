use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

use crate::prelude::{OnUndo, Processing, Undo};

/// Add undo-operations to an app.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UndoPlugin;


impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, undo
                .run_if(any_with_component::<Undo>()),
            );
    }
}


#[inline]
fn undo(
    mut commands: Commands,
    processing: Query<Entity, With<Processing>>,
    undo_query: Query<Entity, With<Undo>>,
    on_undo_query: Query<(Entity, &OnUndo), With<OnUndo>>,
) {
    if processing.is_empty() {
        all_undo_execute(&mut commands, undo_query, on_undo_query);
    } else {
        clear_all_undo(&mut commands, undo_query);
    }
}


fn all_undo_execute(
    commands: &mut Commands,
    undo_query: Query<Entity, With<Undo>>,
    on_undo_query: Query<(Entity, &OnUndo), With<OnUndo>>,
) {
    let mut qs = on_undo_query
        .into_iter()
        .collect::<Vec<(Entity, &OnUndo)>>();

    for undo in undo_query.iter() {
        commands
            .entity(undo)
            .remove::<Undo>();

        if let Some((entity, on_undo)) = qs.pop() {
            on_undo.execute(commands);
            commands
                .entity(entity)
                .remove::<OnUndo>();
        }
    }
}


#[inline]
fn clear_all_undo(
    commands: &mut Commands,
    undo_query: Query<Entity, With<Undo>>,
) {
    for p in undo_query.iter() {
        commands.entity(p).remove::<Undo>();
    }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_util::{new_app, new_entity};

    #[test]
    fn undo_twice_on_1frame() {
        let mut app = new_app();

        let id1 = new_entity(&mut app);
        let id2 = new_entity(&mut app);

        app.undo();
        app.undo();

        app.update();

        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
    }


    #[test]
    fn undo_processing() {
        let mut app = new_app();

        let id1 = new_entity(&mut app);
        let processing = app
            .world
            .spawn(Processing)
            .id();
        app.update();

        app.undo();
        app.undo();
        app.update();

        app
            .world
            .entity_mut(processing)
            .remove::<Processing>();
        app.update();
        assert!(app.world.get_entity(id1).is_some());
    }
}