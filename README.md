# bevy-undo

This crate makes it easy to use the "undo" action on [bevy](https://bevyengine.org/).

[![License: MIT/Apache](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](https://opensource.org/licenses/MIT)
[![Doc](https://docs.rs/bevy_tweening/badge.svg)](https://docs.rs/bevy-undo)
[![Crate](https://img.shields.io/crates/v/bevy-undo.svg)](https://crates.io/crates/bevy-undo)

## Usage

### Basic

If you generate `OnUndo` in `commands` beforehand, the latest `OnUndo` callback will be called when `Undo` is generated.
Each of these components can be easily created with the `on_undo` and `undo` methods.

Here is a simple example that prints `Undo!!!!!!` when you press the R key.when input key `R`

[examples/simple.rs](./examples/simple.rs)

```rust
use bevy::prelude::*;
use bevy_undo::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UndoPlugin) // Please add `UndoPlugin`
        .add_systems(Startup, setup)
        .add_systems(Update, keycode_undo)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands
        .on_undo(|commands: &mut Commands| {
            println!("Undo!!!!!!");
        });
}

fn keycode_undo(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::R) {
        commands.undo();
    }
}
```

You can add one or more entities to the argument of `on_undo`. (maximum 12)

[examples/extension/commands_on_undo_builder.rs](./examples/extension/commands_on_undo_builder.rs)

```rust
use bevy::prelude::*;
use bevy_undo::prelude::*;

fn setup(
    mut commands: Commands
) {
    let id1 = commands
        .spawn_empty()
        .id();
    let id2 = commands
        .spawn_empty()
        .id();
    let id3 = commands
        .spawn_empty()
        .id();

    commands
        .on_undo_builder()
        .add_entity(id1)
        .add_entity(id2)
        .add_entity(id3)
        .on_undo(|commands: &mut Commands, (entity1, entity2, entity3)| {
            println!("undo entity1 = {entity1:?} entity2 = {entity2:?} entity3 = {entity3:?}");
        });
}

```

In the case of `EntityCommand`, the Entity associated with itself is added as an argument.

```rust
use bevy::prelude::*;
use bevy_undo::prelude::*;

fn setup(mut commands: Commands) {
    commands
        .spawn_empty()
        .on_undo(|commands, entity1| {});

    // The code above can be replaced with:
    let id1 = commands
        .spawn_empty()
        .id();
    commands
        .on_undo_builder()
        .add_entity(id1)
        .on_undo(|commands, entity1| {});
}
```

### Processing

Undo is ignored while there is one or more `Processing` in the world.

See [examples/tween/sprite_move.rs](./examples/tween/sprite_move.rs) for details.

## Examples

[here](./examples)

## Features

### tween

Add methods to [bevy_tweening](https://crates.io/crates/bevy_tweening) for undo operations, callbacks on animation
completion, etc

## Compatible Bevy versions

| this crate | bevy   |
|------------|--------|
| 0.1.0      | 0.11.0 |
