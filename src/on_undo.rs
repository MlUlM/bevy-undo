use std::sync::Arc;

use bevy::prelude::*;

pub use crate::on_undo::executor::OnUndoBuilder;
use crate::on_undo::executor::UndoExecutable;

mod executor;
pub(crate) mod handler;


pub mod prelude {
    pub use crate::on_undo::{OnUndo, OnUndoBuilder};
}




#[derive(Component, Clone)]
pub struct OnUndo(Arc<dyn UndoExecutable>);


impl OnUndo {
    #[inline]
    pub const fn builder() -> OnUndoBuilder {
        OnUndoBuilder::new()
    }


    #[inline]
    #[allow(clippy::self_named_constructors)]
    pub fn on_undo(handler: impl Fn(&mut Commands) + Send + Sync + 'static) -> Self {
        Self::builder().on_undo(handler)
    }


    #[inline]
    pub(crate) fn new(exe: impl UndoExecutable + 'static) -> Self {
        Self(Arc::new(exe))
    }


    #[inline]
    pub(crate) fn execute(&self, commands: &mut Commands) {
        self.0.undo(commands);
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::App;

    use crate::prelude::*;
    use crate::test_util::new_entity;

    #[test]
    fn once_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id = new_entity(&mut app);
        // Undo is not executed unless UndoExecution is issued
        app.update();
        assert!(app.world.get_entity(id).is_some());

        app.undo();
        app.update();
        assert!(app.world.get_entity(id).is_none());
    }


    #[test]
    fn two_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id1 = new_entity(&mut app);
        let id2 = new_entity(&mut app);

        // Undo is not executed unless UndoExecution is issued
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_some());

        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());

        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
    }


    #[test]
    fn three_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id1 = new_entity(&mut app);
        let id2 = new_entity(&mut app);

        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());

        let id3 = new_entity(&mut app);
        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());
        assert!(app.world.get_entity(id3).is_none());

        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
        assert!(app.world.get_entity(id3).is_none());
    }


    #[test]
    fn non_attach() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);
        let id1 = app.world.spawn_empty().id();
        let id2 = app.world.spawn_empty().id();

        let on_undo = OnUndoBuilder::new()
            .add_entity(id1)
            .add_entity(id2)
            .on_undo(|cmd, (id1, id2)| {
                cmd.entity(id1).despawn();
                cmd.entity(id2).despawn();
            });
        app
            .world
            .entity_mut(id1)

            .insert(on_undo);
        app.update();

        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
    }


    #[test]
    fn many_spawn_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id = new_entity(&mut app);
        // Undo is not executed unless UndoExecution is issued
        app.update();
        assert!(app.world.get_entity(id).is_some());

        app.undo();
        app.update();
        assert!(app.world.get_entity(id).is_none());
    }
}