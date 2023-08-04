use bevy::prelude::{Commands, Entity};

use crate::on_undo::executor::UndoExecutable;
use crate::on_undo::handler::UndoHandler;
use crate::on_undo::OnUndo;

pub(crate) struct One {
    undo: Box<dyn UndoHandler<Entity>>,
    entity: Entity,
}


impl One {
    #[inline]
    pub fn create(
        entity: Entity,
        handler: impl UndoHandler<Entity>,
    ) -> OnUndo {
        OnUndo::new(Self {
            undo: Box::new(handler),
            entity,
        })
    }
}


impl UndoExecutable for One {
    #[inline]
    fn undo(&self, commands: &mut Commands) {
        self.undo.handle(commands, self.entity);
    }
}