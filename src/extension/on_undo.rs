use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{Commands, Entity};

use crate::on_undo::OnUndoBuilder;

pub trait CommandsOnUndoExt {
    fn on_undo(&mut self, undo: impl Fn(&mut Commands) + Send + Sync + 'static);
}


impl<'w, 's> CommandsOnUndoExt for Commands<'w, 's> {
    #[inline]
    fn on_undo(&mut self, undo: impl Fn(&mut Commands) + Send + Sync + 'static) {
        self.spawn(OnUndoBuilder::new().on_undo(undo));
    }
}


pub trait EntityCommandsOnUndoExt {
    fn on_undo(&mut self, undo: impl Fn(&mut Commands, Entity) + Send + Sync + 'static);


    fn on_undo_with_entity_commands(&mut self, undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static) {
        self.on_undo(move |cmd, entity| {
            undo(&mut cmd.entity(entity));
        });
    }
}


impl<'w> EntityCommandsOnUndoExt for EntityMut<'w> {
    #[inline]
    fn on_undo(&mut self, undo: impl Fn(&mut Commands, Entity) + Send + Sync + 'static) {
        let id = self.id();
        unsafe {
            self
                .world_mut()
                .spawn(OnUndoBuilder::new()
                    .add_entity(id)
                    .on_undo(undo)
                );
        }
    }
}


impl<'w, 's, 'a> EntityCommandsOnUndoExt for EntityCommands<'w, 's, 'a> {
    #[inline]
    fn on_undo(&mut self, undo: impl Fn(&mut Commands, Entity) + Send + Sync + 'static) {
        let id = self.id();
        self
            .commands()
            .spawn(OnUndoBuilder::new()
                .add_entity(id)
                .on_undo(undo)
            );
    }
}