use bevy::app::{App, Plugin, Update};

/// Add undo-operations to an app.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UndoTweenPlugin;


impl Plugin for UndoTweenPlugin {
    #[inline]
    fn build(&self, app: &mut App) {
        use crate::extension::prelude::tween_completed;
        app.add_systems(Update, tween_completed);
    }
}
