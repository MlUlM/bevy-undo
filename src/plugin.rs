use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

use crate::prelude::{OnUndo, Processing, Undo};

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UndoPlugin;


impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, undo
                .run_if(any_with_component::<OnUndo>()
                    .and_then(any_with_component::<Undo>())
                    .and_then(not(any_with_component::<Processing>()))
                ),
            );


        #[cfg(feature = "tween")]
        {
            use crate::extension::prelude::tween_completed;
            app.add_systems(Update, tween_completed
                .before(undo)
            );
        }
    }
}


fn undo(
    mut commands: Commands,
    undo_query: Query<Entity, Added<Undo>>,
    on_undo_query: Query<(Entity, &OnUndo), With<OnUndo>>,
) {
    let mut qs = on_undo_query
        .into_iter()
        .collect::<Vec<(Entity, &OnUndo)>>();

    for undo in undo_query.iter() {
        commands
            .entity(undo)
            .remove::<Undo>();

        let Some((entity, on_undo)) = qs.pop() else { return; };
        on_undo.execute(&mut commands);
        commands
            .entity(entity)
            .remove::<OnUndo>();
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::App;

    use crate::prelude::*;
    use crate::test_util::{new_entity, undo};

    #[test]
    fn undo_twice_on_1frame() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id1 = new_entity(&mut app);
        let id2 = new_entity(&mut app);

        undo(&mut app);
        undo(&mut app);

        app.update();

        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
    }
}