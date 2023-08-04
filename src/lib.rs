#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub use processing::Processing;
pub use undo::Undo;

use crate::on_undo::OnUndo;

pub mod on_undo;
pub mod extension;
mod undo;
mod processing;


pub mod prelude {
    pub use crate::{
        extension::prelude::*, on_undo::prelude::*, processing::Processing, undo::Undo,
        UndoPlugin,
    };
}


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
            use extension::tween::tween_completed;
            app.add_systems(Update, tween_completed);
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
        on_undo.execute(&mut commands, entity);
        commands
            .entity(entity)
            .remove::<OnUndo>();
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::App;
    use bevy::prelude::{Entity, SpriteBundle};

    use crate::{Undo, UndoPlugin};

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


    pub(crate) fn undo(app: &mut App) {
        app.world.spawn(Undo);
    }


    pub(crate) fn new_entity(app: &mut App) -> Entity {
        let mut entity = app
            .world
            .spawn_empty();
        entity
            .insert(SpriteBundle::default())
            .on_undo(|command| {
                command.despawn();
            });

        entity.id()
    }
}