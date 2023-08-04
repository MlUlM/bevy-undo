use bevy::prelude::{Commands, Entity};

use crate::on_undo::executor::UndoExecutable;
use crate::on_undo::handler::UndoHandler;
use crate::on_undo::OnUndo;

pub(crate) struct Twelve {
    handler: Box<dyn UndoHandler<(
        Entity,
        Entity,
        Entity,
        Entity,
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
    entity9: Entity,
    entity10: Entity,
    entity11: Entity,
    entity12: Entity,
}


impl Twelve {
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
        entity9: Entity,
        entity10: Entity,
        entity11: Entity,
        entity12: Entity,
        handler: impl UndoHandler<(
            Entity,
            Entity,
            Entity,
            Entity,
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
            entity9,
            entity10,
            entity11,
            entity12,
            handler: Box::new(handler),
        })
    }
}


impl UndoExecutable for Twelve {
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
            self.entity8,
            self.entity9,
            self.entity10,
            self.entity11,
            self.entity12,
        ));
    }
}