#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_tweening::*;
use bevy_tweening::lens::TransformPositionLens;

use bevy_undo::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UndoPlugin)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keycode_undo.run_if(not(any_with_component::<Processing>())))
        .add_systems(Update, keycode_move.run_if(not(any_with_component::<Processing>())))
        .add_systems(Update, undoing)
        .run();
}


#[derive(Debug, Copy, Clone, Component)]
struct Movable;


#[derive(Debug, Copy, Clone, Component)]
struct MovePos(Vec3);


fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE_RED,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        })
        .insert(Movable);
}


fn keycode_undo(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::R) {
        commands.undo();
    }
}


fn keycode_move(
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
    q: Query<(Entity, &Transform), (With<Transform>, With<Movable>)>,
) {
    let mut start_move = move |relative: Vec3| {
        let (entity, transform) = q.single();
        let pos = transform.translation;
        let tween = bevy_tweening::Tween::new(
            EaseMethod::Linear,
            std::time::Duration::from_secs(1),
            TransformPositionLens {
                start: transform.translation,
                end: transform.translation + relative,
            },
        )
            .spawn_processing_until_completed(&mut commands)
            .on_undo(&mut commands, move |cmd| {
                cmd.insert(MovePos(pos));
            });

        commands
            .entity(entity)
            .insert(Animator::new(tween));
    };

    if key.just_pressed(KeyCode::Left) {
        start_move(Vec3::NEG_X * 100.);
    } else if key.just_pressed(KeyCode::Up) {
        start_move(Vec3::Y * 100.);
    } else if key.just_pressed(KeyCode::Right) {
        start_move(Vec3::X * 100.);
    } else if key.just_pressed(KeyCode::Down) {
        start_move(Vec3::NEG_Y * 100.);
    }
}


fn undoing(
    mut commands: Commands,
    q: Query<(Entity, &Transform, &MovePos), (
        With<Transform>,
        With<Movable>,
        Added<MovePos>
    )>,
) {
    for (entity, transform, MovePos(move_pos)) in q.iter() {
        let tween = bevy_tweening::Tween::new(
            EaseMethod::Linear,
            std::time::Duration::from_secs(1),
            TransformPositionLens {
                start: transform.translation,
                end: *move_pos,
            },
        )
            .spawn_processing_until_completed(&mut commands);

        commands
            .entity(entity)
            .remove::<MovePos>()
            .insert(Animator::new(tween));
    }
}
