use bevy::prelude::Component;

/// Undo is ignored while there is one or more `Processing` in the world.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Component)]
pub struct Processing;