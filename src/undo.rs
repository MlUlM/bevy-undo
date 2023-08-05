use bevy::prelude::Component;

/// Component to request undo action.
///
/// This component is destroyed when the actual undo operation is performed.
///
/// It can be spawned multiple times in one frame. In the next frame, perform an undo operation on as many as were spawned.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Component)]
pub struct Undo;