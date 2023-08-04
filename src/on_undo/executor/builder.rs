use bevy::prelude::{Commands, Component, Entity};

use crate::on_undo::executor::five::Five;
use crate::on_undo::executor::four::Four;
use crate::on_undo::executor::one::One;
use crate::on_undo::executor::single::Single;
use crate::on_undo::executor::six::Six;
use crate::on_undo::executor::three::Three;
use crate::on_undo::executor::two::Two;
use crate::on_undo::OnUndo;

#[derive(Component, Default)]
pub struct OnUndoBuilder<const N: usize = 0> {
    entity1: Option<Entity>,
    entity2: Option<Entity>,
    entity3: Option<Entity>,
    entity4: Option<Entity>,
    entity5: Option<Entity>,
    entity6: Option<Entity>,
}


impl OnUndoBuilder<0> {
    #[inline]
    pub(crate) const fn new() -> OnUndoBuilder<0> {
        OnUndoBuilder {
            entity1: None,
            entity2: None,
            entity3: None,
            entity4: None,
            entity5: None,
            entity6: None,
        }
    }


    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<1> {
        OnUndoBuilder {
            entity1: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands) + Send + Sync + 'static) -> OnUndo {
        Single::create(handler)
    }
}


impl OnUndoBuilder<1> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<2> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, Entity) + Send + Sync + 'static) -> OnUndo {
        One::create(self.entity1.unwrap(), handler)
    }
}


impl OnUndoBuilder<2> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<3> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Two::create(self.entity1.unwrap(), self.entity2.unwrap(), handler)
    }
}


impl OnUndoBuilder<3> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<4> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Three::create(self.entity1.unwrap(), self.entity2.unwrap(), self.entity3.unwrap(), handler)
    }
}


impl OnUndoBuilder<4> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<5> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (Entity, Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Four::create(self.entity1.unwrap(), self.entity2.unwrap(), self.entity3.unwrap(), self.entity4.unwrap(), handler)
    }
}


impl OnUndoBuilder<5> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<6> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: Some(entity),
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (Entity, Entity, Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Five::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            handler,
        )
    }
}


impl OnUndoBuilder<6> {
    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (Entity, Entity, Entity, Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Six::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            handler,
        )
    }
}
