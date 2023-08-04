use bevy::ecs::system::EntityCommands;

use crate::extension::on_undo::builder::OnUndoBuilderWithCommands;

pub trait EntityCommandsOnUndoBuilderExt<'w, 's, 'd> {
    fn on_undo_builder(&'d mut self) -> OnUndoBuilderWithCommands<'w, 's, 'd, 1>;
}


impl<'w, 's, 'd> EntityCommandsOnUndoBuilderExt<'w, 's, 'd> for EntityCommands<'w, 's, 'd> {
    #[inline]
    fn on_undo_builder(&'d mut self) -> OnUndoBuilderWithCommands<'w, 's, 'd, 1> {
        let id1 = self.id();
        OnUndoBuilderWithCommands::new(self.commands())
            .add_entity(id1)
    }
}



