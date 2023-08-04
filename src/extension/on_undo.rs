use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{Commands, Entity};

use builder::OnUndoBuilderWithCommands;

use crate::on_undo::OnUndoBuilder;

mod builder;
mod entity_commands_on_undo_builder;


pub mod prelude {
    pub use crate::extension::on_undo::{CommandsOnUndoExt, EntityCommandsOnUndoExt};
    pub use crate::extension::on_undo::entity_commands_on_undo_builder::EntityCommandsOnUndoBuilderExt;
}


pub trait CommandsOnUndoExt<'w, 's, 'd> {
    fn on_undo(&mut self, on_undo: impl Fn(&mut Commands) + Send + Sync + 'static);

    fn on_undo_builder(&'d mut self) -> OnUndoBuilderWithCommands<'w, 's, 'd>;
}


impl<'w, 's, 'd> CommandsOnUndoExt<'w, 's, 'd> for Commands<'w, 's> {
    #[inline]
    fn on_undo(&mut self, on_undo: impl Fn(&mut Commands) + Send + Sync + 'static) {
        self.spawn(OnUndoBuilder::new().on_undo(on_undo));
    }


    #[inline]
    fn on_undo_builder(&'d mut self) -> OnUndoBuilderWithCommands<'w, 's, 'd> {
        OnUndoBuilderWithCommands::new(self)
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




