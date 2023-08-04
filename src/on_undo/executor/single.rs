use bevy::prelude::Commands;

use crate::on_undo::executor::UndoExecutable;
use crate::on_undo::handler::UndoHandler;
use crate::on_undo::OnUndo;

pub(crate) struct Single(Box<dyn UndoHandler<()>>);


impl Single {
    #[inline]
    pub(crate) fn create(undo: impl Fn(&mut Commands) + Send + Sync + 'static) -> OnUndo {
        OnUndo::new(Self(Box::new(undo)))
    }
}


impl UndoExecutable for Single {
    #[inline]
    fn undo(&self, commands: &mut Commands) {
        self.0.handle(commands, ());
    }
}