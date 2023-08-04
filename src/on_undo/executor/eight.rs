use bevy::prelude::{Commands, Entity};

use crate::on_undo::executor::UndoExecutable;
use crate::on_undo::handler::UndoHandler;
use crate::on_undo::OnUndo;

pub(crate) struct Eight {
    handler: Box<dyn UndoHandler<(
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
    )>>,
    entity1: Entity,
    entity2: Entity,
    entity3: Entity,
    entity4: Entity,
    entity5: Entity,
    entity6: Entity,
    entity7: Entity,
    entity8: Entity,
}


impl Eight {
    #[inline]
    pub(crate) fn create(
        entity1: Entity,
        entity2: Entity,
        entity3: Entity,
        entity4: Entity,
        entity5: Entity,
        entity6: Entity,
        entity7: Entity,
        entity8: Entity,
        handler: impl UndoHandler<(
            Entity,
            Entity,
            Entity,
            Entity,
            Entity,
            Entity,
            Entity,
            Entity,
        )> + 'static,
    ) -> OnUndo {
        OnUndo::new(Self {
            entity1,
            entity2,
            entity3,
            entity4,
            entity5,
            entity6,
            entity7,
            entity8,
            handler: Box::new(handler),
        })
    }
}


impl UndoExecutable for Eight {
    #[inline]
    fn undo(&self, commands: &mut Commands) {
        self.handler.handle(commands, (
            self.entity1,
            self.entity2,
            self.entity3,
            self.entity4,
            self.entity5,
            self.entity6,
            self.entity7,
            self.entity8
        ));
    }
}