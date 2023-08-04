use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Component, Entity, EventReader, Query, With};
use bevy::utils::HashMap;
use bevy_tweening::{Tween, TweenCompleted};

use crate::extension::tween::processing::TweenProcessing;
use crate::prelude::*;

mod processing;

pub trait TweenOnUndoExt<T>: Sized {
    fn spawn_processing_and_remove_completed(
        self,
        commands: &mut Commands,
    ) -> Tween<T>;


    fn with_completed_entity_commands(
        self,
        commands: &mut Commands,
        on_completed: impl Fn(&mut EntityCommands) + Send + Sync + 'static + Clone,
    ) -> Tween<T>;


    fn on_undo(
        self,
        commands: &mut Commands,
        on_undo: OnUndo,
    ) -> Tween<T>;


    #[inline]
    fn remove_processing_on_completed(self, commands: &mut Commands) -> Tween<T> {
        self.with_completed_entity_commands(commands, |cmd| {
            cmd.remove::<Processing>();
        })
    }
}


#[doc(hidden)]
#[derive(Component)]
pub(crate) struct TweenOnUndo {
    user_data: u64,
    on_undo: OnUndo,
}


#[doc(hidden)]
#[derive(Component)]
pub(crate) struct TweenOnCompleted {
    user_data: u64,
    on_completed: Box<dyn TweenOnUndoHandle>,
}


struct TweenOnUndoHandler<T> {
    handler: T,
}


impl<T> TweenOnUndoHandle for TweenOnUndoHandler<T>
    where T: Fn(&mut EntityCommands) + Send + Sync + 'static + Clone
{
    fn handler(&self) -> Box<dyn Fn(&mut EntityCommands) + Send + Sync + 'static> {
        Box::new(self.handler.clone())
    }
}

// No particular meaning.
const USER_DATA: u64 = 3611388714;


trait TweenOnUndoHandle: Send + Sync + 'static {
    fn handler(&self) -> Box<dyn Fn(&mut EntityCommands) + Send + Sync + 'static>;
}


impl<T: Component> TweenOnUndoExt<T> for Tween<T> {
    fn spawn_processing_and_remove_completed(self, commands: &mut Commands) -> Tween<T> {
        commands
            .spawn_empty()
            .insert(Processing)
            .insert(TweenProcessing(USER_DATA));

        self.with_completed_event(USER_DATA)
    }


    fn with_completed_entity_commands(self, commands: &mut Commands, on_completed: impl Fn(&mut EntityCommands) + Send + Sync + 'static + Clone) -> Tween<T> {
        commands.spawn(TweenOnCompleted {
            user_data: USER_DATA,
            on_completed: Box::new(TweenOnUndoHandler { handler: on_completed }),
        });

        self.with_completed_event(USER_DATA)
    }


    fn on_undo(
        self,
        commands: &mut Commands,
        on_undo: OnUndo,
    ) -> Tween<T> {
        commands.spawn(TweenOnUndo {
            user_data: USER_DATA,
            on_undo,
        });

        self.with_completed_event(USER_DATA)
    }
}


pub(crate) fn tween_completed(
    mut reader: EventReader<TweenCompleted>,
    mut commands: Commands,
    on_undo_query: Query<(Entity, &TweenOnUndo), With<TweenOnUndo>>,
    on_completed_query: Query<(Entity, &TweenOnCompleted), With<TweenOnCompleted>>,
    on_processing_query: Query<(Entity, &Processing, &TweenProcessing), (With<Processing>, With<TweenProcessing>)>,
) {
    let mut on_undo_qs = on_undo_query
        .iter()
        .map(|(e, on_undo)| (on_undo.user_data, (e, on_undo)))
        .collect::<HashMap<u64, (Entity, &TweenOnUndo)>>();

    let mut on_completed_qs = on_completed_query
        .iter()
        .map(|(e, on_completed)| (on_completed.user_data, (e, on_completed)))
        .collect::<HashMap<u64, (Entity, &TweenOnCompleted)>>();

    let mut ps_qs = on_processing_query
        .iter()
        .map(|(entity, _, tween_ps)| (tween_ps.0, entity))
        .collect::<HashMap<u64, Entity>>();

    for TweenCompleted { user_data, entity } in reader.iter() {
        if let Some(processing_id) = ps_qs.remove(user_data) {
            commands
                .entity(processing_id)
                .remove::<Processing>()
                .remove::<TweenProcessing>();
        }

        if let Some((on_completed_id, on_completed)) = on_completed_qs.remove(user_data) {
            on_completed.on_completed.handler()(&mut commands.entity(*entity));
            commands
                .entity(on_completed_id)
                .remove::<TweenOnCompleted>();
        }

        if let Some((on_undo_id, tween_on_undo)) = on_undo_qs.remove(user_data) {
            commands
                .entity(on_undo_id)
                .remove::<TweenOnUndo>();

            commands
                .spawn(tween_on_undo.on_undo.clone());
        }
    }
}