use bevy::prelude::Commands;

pub use builder::OnUndoBuilder;

mod builder;
mod single;
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;

pub(crate) trait UndoExecutable: Send + Sync {
    fn undo(&self, commands: &mut Commands);
}





