use bevy::prelude::{Commands, Component, Entity};

use crate::on_undo::executor::eight::Eight;
use crate::on_undo::executor::eleven::Eleven;
use crate::on_undo::executor::five::Five;
use crate::on_undo::executor::four::Four;
use crate::on_undo::executor::nine::Nine;
use crate::on_undo::executor::one::One;
use crate::on_undo::executor::seven::Seven;
use crate::on_undo::executor::single::Single;
use crate::on_undo::executor::six::Six;
use crate::on_undo::executor::ten::Ten;
use crate::on_undo::executor::three::Three;
use crate::on_undo::executor::twelve::Twelve;
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
    entity7: Option<Entity>,
    entity8: Option<Entity>,
    entity9: Option<Entity>,
    entity10: Option<Entity>,
    entity11: Option<Entity>,
    entity12: Option<Entity>,
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
            entity7: None,
            entity8: None,
            entity9: None,
            entity10: None,
            entity11: None,
            entity12: None,
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
            ..Default::default()
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
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<7> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: self.entity6,
            entity7: Some(entity),
            ..Default::default()
        }
    }


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


impl OnUndoBuilder<7> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<8> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: self.entity6,
            entity7: self.entity7,
            entity8: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
    )) + Send + Sync + 'static) -> OnUndo {
        Seven::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            self.entity7.unwrap(),
            handler,
        )
    }
}


impl OnUndoBuilder<8> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<9> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: self.entity6,
            entity7: self.entity7,
            entity8: self.entity8,
            entity9: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
    )) + Send + Sync + 'static) -> OnUndo {
        Eight::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            self.entity7.unwrap(),
            self.entity8.unwrap(),
            handler,
        )
    }
}


impl OnUndoBuilder<9> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<10> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: self.entity6,
            entity7: self.entity7,
            entity8: self.entity8,
            entity9: self.entity9,
            entity10: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
        Entity,
    )) + Send + Sync + 'static) -> OnUndo {
        Nine::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            self.entity7.unwrap(),
            self.entity8.unwrap(),
            self.entity9.unwrap(),
            handler,
        )
    }
}


impl OnUndoBuilder<10> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<11> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: self.entity6,
            entity7: self.entity7,
            entity8: self.entity8,
            entity9: self.entity9,
            entity10: self.entity10,
            entity11: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (
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
    )) + Send + Sync + 'static) -> OnUndo {
        Ten::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            self.entity7.unwrap(),
            self.entity8.unwrap(),
            self.entity9.unwrap(),
            self.entity10.unwrap(),
            handler,
        )
    }
}


impl OnUndoBuilder<11> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<12> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: self.entity6,
            entity7: self.entity7,
            entity8: self.entity8,
            entity9: self.entity9,
            entity10: self.entity10,
            entity11: self.entity11,
            entity12: Some(entity),
        }
    }


    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (
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
    )) + Send + Sync + 'static) -> OnUndo {
        Eleven::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            self.entity7.unwrap(),
            self.entity8.unwrap(),
            self.entity9.unwrap(),
            self.entity10.unwrap(),
            self.entity11.unwrap(),
            handler,
        )
    }
}


impl OnUndoBuilder<12> {
    #[inline]
    pub fn on_undo(self, handler: impl Fn(&mut Commands, (
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
    )) + Send + Sync + 'static) -> OnUndo {
        Twelve::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            self.entity7.unwrap(),
            self.entity8.unwrap(),
            self.entity9.unwrap(),
            self.entity10.unwrap(),
            self.entity11.unwrap(),
            self.entity12.unwrap(),
            handler,
        )
    }
}