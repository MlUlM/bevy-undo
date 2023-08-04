use bevy::prelude::{Commands, Entity};

use crate::prelude::OnUndoBuilder;

pub struct OnUndoBuilderWithCommands<'w, 's, 'd, const N: usize = 0> {
    commands: &'d mut Commands<'w, 's>,
    builder: OnUndoBuilder<N>,
}


impl<'w, 's, 'd> OnUndoBuilderWithCommands<'w, 's, 'd, 0> {
    #[inline]
    pub(crate) fn new(commands: &'d mut Commands<'w, 's>) -> OnUndoBuilderWithCommands<'w, 's, 'd, 0> {
        Self {
            commands,
            builder: OnUndoBuilder::new(),
        }
    }
}


macro_rules! builder {
       ($n: expr, $next: expr) => {
        impl<'w, 's, 'd> OnUndoBuilderWithCommands<'w, 's, 'd, $n> {
            #[inline]
            pub fn add_entity(self, entity: bevy::prelude::Entity) -> OnUndoBuilderWithCommands<'w, 's, 'd, $next>{
                OnUndoBuilderWithCommands{
                    commands: self.commands,
                    builder: self.builder.add_entity(entity)
                }
            }


            #[inline]
            pub fn on_undo(self, handler: impl Fn(&mut Commands) + Send + Sync + 'static){
                self.commands.spawn(self.builder.on_undo(handler));
            }
        }
    };



    ($n: expr, $next: expr, Entity) => {
        impl<'w, 's, 'd> OnUndoBuilderWithCommands<'w, 's, 'd, $n> {
            #[inline]
            pub fn add_entity(self, entity: bevy::prelude::Entity) -> OnUndoBuilderWithCommands<'w, 's, 'd, $next>{
                OnUndoBuilderWithCommands{
                    commands: self.commands,
                    builder: self.builder.add_entity(entity)
                }
            }


            #[inline]
            pub fn on_undo(self, handler: impl Fn(&mut Commands, Entity) + Send + Sync + 'static){
                self.commands.spawn(self.builder.on_undo(handler));
            }
        }
    };


    ($n: expr, $next: expr, Entity $(, $entity: ident)*) => {
        impl<'w, 's, 'd> OnUndoBuilderWithCommands<'w, 's, 'd, $n> {
            #[inline]
            pub fn add_entity(self, entity: bevy::prelude::Entity) -> OnUndoBuilderWithCommands<'w, 's, 'd, $next>{
                OnUndoBuilderWithCommands{
                    commands: self.commands,
                    builder: self.builder.add_entity(entity)
                }
            }


            #[inline]
            pub fn on_undo(self, handler: impl Fn(&mut Commands, (Entity $(,$entity)*) ) + Send + Sync + 'static){
                self.commands.spawn(self.builder.on_undo(handler));
            }
        }
    };
}


builder!(0, 1);
builder!(1, 2, Entity);
builder!(2, 3, Entity, Entity);
builder!(3, 4, Entity, Entity, Entity);
builder!(4, 5, Entity, Entity, Entity, Entity);
builder!(5, 6, Entity, Entity, Entity, Entity, Entity);
builder!(6, 7, Entity, Entity, Entity, Entity, Entity, Entity);
builder!(7, 8, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
builder!(8, 9, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
builder!(9, 10, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
builder!(10, 11, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
builder!(11, 12, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);

