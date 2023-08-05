use bevy::app::App;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::Commands;

use crate::Undo;

pub trait CommandsUndoExt {
    /// Spawns an empty entity with [`Undo`] inserted.
    fn undo(&mut self);
}


impl<'w, 's> CommandsUndoExt for Commands<'w, 's> {
    #[inline]
    fn undo(&mut self) {
        self.spawn(Undo);
    }
}


impl<'w> CommandsUndoExt for EntityMut<'w> {
    #[inline]
    fn undo(&mut self) {
        unsafe {
            self.world_mut().spawn(Undo);
        }
    }
}


impl CommandsUndoExt for App {
    fn undo(&mut self) {
        self.world.spawn(Undo);
    }
}


impl<'w, 's, 'a> CommandsUndoExt for EntityCommands<'w, 's, 'a> {
    #[inline]
    fn undo(&mut self) {
        self.commands().undo();
    }
}