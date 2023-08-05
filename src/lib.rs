#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]


use undo::Undo;

use crate::on_undo::OnUndo;

mod on_undo;
mod extension;
mod undo;
mod processing;
mod plugin;


pub mod prelude {
    pub use crate::{
        extension::prelude::*, on_undo::prelude::*, plugin::UndoPlugin, processing::Processing,
        undo::Undo,
    };
}


#[cfg(test)]
pub(crate) mod test_util {
    use bevy::app::App;
    use bevy::prelude::{Entity, SpriteBundle};
    use crate::plugin::UndoPlugin;
    use crate::prelude::EntityCommandsOnUndoExt;

    pub(crate) fn new_app() -> App {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        app
    }


    pub(crate) fn new_entity(app: &mut App) -> Entity {
        let mut entity = app
            .world
            .spawn_empty();

        entity
            .insert(SpriteBundle::default())
            .on_undo_with_entity_commands(|command| {
                command.despawn();
            });

        entity.id()
    }
}
