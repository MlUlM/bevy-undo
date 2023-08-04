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
mod seven;
mod eight;
mod nine;
mod ten;
mod eleven;
mod twelve;

pub(crate) trait UndoExecutable: Send + Sync {
    fn undo(&self, commands: &mut Commands);
}





