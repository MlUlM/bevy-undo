use bevy::prelude::{Commands, Entity};

pub(crate) trait UndoHandler<T>: Send + Sync + 'static
    where T: Send + Sync + 'static
{
    fn handle(&self, commands: &mut Commands, entities: T);
}


impl<F> UndoHandler<()> for F
    where F: Fn(&mut Commands) + Send + Sync + 'static
{
    #[inline]
    fn handle(&self, commands: &mut Commands, _: ()) {
        self(commands);
    }
}


impl<F> UndoHandler<Entity> for F
    where F: Fn(&mut Commands, Entity) + Send + Sync + 'static
{
    #[inline]
    fn handle(&self, commands: &mut Commands, entities: Entity) {
        self(commands, entities)
    }
}


macro_rules! tuples {
    ($($entities: ty),*) => {
        impl<F> UndoHandler<($($entities),*)> for F
            where F: Fn(&mut Commands, ($($entities),*)) + Send + Sync + 'static
        {
            #[inline]
            fn handle(&self, commands: &mut Commands, entities: ($($entities),*)) {
                self(commands, entities)
            }
        }
    };
}


tuples!(Entity, Entity);
tuples!(Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);

