use bevy::prelude::{Commands, Entity};

use crate::on_undo::executor::UndoExecutable;
use crate::on_undo::handler::UndoHandler;
use crate::on_undo::OnUndo;

pub(crate) struct Two {
    undo: Box<dyn UndoHandler<(Entity, Entity)>>,
    entity1: Entity,
    entity2: Entity,
}


impl Two {
    #[inline]
    pub fn create(
        entity1: Entity,
        entity2: Entity,
        handler: impl UndoHandler<(Entity, Entity)>,
    ) -> OnUndo {
        OnUndo::new(Self {
            undo: Box::new(handler),
            entity1,
            entity2,
        })
    }
}


impl UndoExecutable for Two {
    #[inline]
    fn undo(&self, commands: &mut Commands) {
        self.undo.handle(commands, (self.entity1, self.entity2));
    }
}